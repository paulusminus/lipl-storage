use futures_util::future::BoxFuture;
// use convert::to_toml;
use lipl_core::transaction::{OptionalTransaction, start_log_thread};
use std::fmt::{Debug, Display};
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::task::JoinHandle;

use constant::{LYRIC_EXTENSION, TOML_EXTENSION};
use fs::IO;
use futures_channel::mpsc;
use futures_util::{FutureExt, StreamExt, TryFutureExt, TryStreamExt};
pub use lipl_core::error::{Error, ErrorExtension};
use lipl_core::vec_ext::VecExt;
use lipl_core::{LiplRepo, Lyric, Playlist, Summary, ToRepo, Uuid, transaction::Request};
use request::{delete_by_id, post, select, select_by_id};

pub mod constant;
// mod convert;
mod fs;
mod io;
mod request;

#[derive(Clone)]
pub struct FileRepoConfig {
    pub path: String,
}

impl FromStr for FileRepoConfig {
    type Err = lipl_core::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.is_dir().map(|_| FileRepoConfig { path: s.into() })
    }
}

impl ToRepo for FileRepoConfig {
    type Repo = FileRepo;
    async fn to_repo(self) -> lipl_core::Result<Self::Repo> {
        FileRepo::new(self.path).await.map_err(Into::into)
    }
}

#[derive(Clone)]
pub struct FileRepo {
    tx: mpsc::Sender<Request>,
    path: String,
    _join_handle: Arc<JoinHandle<bool>>,
}

impl Debug for FileRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileRepo: {}", self.path)
    }
}

fn check_members(
    playlist: &Playlist,
    lyric_ids: &[Uuid],
) -> impl futures_util::Future<Output = Result<(), Error>> + use<> {
    if let Some(member) = playlist
        .members
        .iter()
        .find(|member| !lyric_ids.contains(member))
    {
        futures_util::future::ready(Err(Error::PlaylistInvalidMember(
            playlist.id.to_string(),
            member.to_string(),
        )))
    } else {
        futures_util::future::ready(Ok(()))
    }
}

fn send<T, D: Display>(
    s: futures_channel::oneshot::Sender<T>,
    error_message: D,
) -> impl FnOnce(T) -> Result<(), Error> {
    move |t| {
        s.send(t)
            .map_err(|_| Error::SendFailed(error_message.to_string()))
    }
}

async fn handle_request<P, Q>(
    request: Request,
    source_dir: String,
    lyric_path: P,
    playlist_path: Q,
) -> Result<(), lipl_core::Error>
where
    P: Fn(&Uuid) -> PathBuf,
    Q: Fn(&Uuid) -> PathBuf,
{
    match request {
        Request::Stop(sender) => {
            async { Ok::<(), lipl_core::Error>(()) }
                .map(send(sender, "Stop"))
                .await?;
            Err(lipl_core::Error::Stop)
        }
        Request::LyricSummaries(sender) => {
            io::get_list(&source_dir, LYRIC_EXTENSION, io::get_lyric_summary)
                .err_into()
                .map(send(sender, "LyricSummaries"))
                .await
        }
        Request::LyricList(sender) => {
            io::get_list(&source_dir, LYRIC_EXTENSION, io::get_lyric)
                .err_into()
                .map(send(sender, "LyricList"))
                .await
        }
        Request::LyricItem(uuid, sender) => {
            io::get_lyric(lyric_path(&uuid))
                .err_into()
                .map(send(sender, format!("LyricItem {uuid}")))
                .await
        }
        Request::LyricDelete(uuid, sender) => {
            async {
                let playlists = lyric_path(&uuid)
                    .remove()
                    .and_then(|_| io::get_list(&source_dir, TOML_EXTENSION, io::get_playlist))
                    .await?;
                for mut playlist in playlists {
                    if playlist.members.contains(&uuid) {
                        playlist.members = playlist.members.without(&uuid);
                        io::post_item(
                            source_dir.full_path(&uuid.to_string(), TOML_EXTENSION),
                            playlist,
                        )
                        .await?;
                    }
                }
                Ok::<(), lipl_core::Error>(())
            }
            .map(send(sender, format!("LyricDelete {uuid}")))
            .await
        }
        Request::LyricPost(lyric, sender) => {
            let path = lyric_path(&lyric.id);
            io::post_item(&path, lyric.clone())
                .and_then(|_| io::get_lyric(&path))
                .err_into()
                .map(send(sender, format!("LyricPost {}", &lyric.title)))
                .await
        }
        Request::PlaylistSummaries(sender) => {
            io::get_list(&source_dir, TOML_EXTENSION, io::get_playlist)
                .map_ok(lipl_core::to_summaries)
                .err_into()
                .map(send(sender, "PlaylistSummaries"))
                .await
        }
        Request::PlaylistList(sender) => {
            io::get_list(&source_dir, TOML_EXTENSION, io::get_playlist)
                .err_into()
                .map(send(sender, "PlaylistList"))
                .await
        }
        Request::PlaylistItem(uuid, sender) => {
            io::get_playlist(playlist_path(&uuid))
                .err_into()
                .map(send(sender, format!("PlaylistItem {uuid}")))
                .await
        }
        Request::PlaylistDelete(uuid, sender) => {
            let path = playlist_path(&uuid);
            path.remove()
                .err_into()
                .map(send(sender, format!("PlaylistDelete {uuid}")))
                .await
        }
        Request::PlaylistPost(playlist, sender) => {
            io::get_list(&source_dir, LYRIC_EXTENSION, io::get_lyric_summary)
                .map_ok(|summaries| lipl_core::ids(summaries.into_iter()))
                .and_then(|ids| check_members(&playlist, &ids))
                .and_then(|_| io::post_item(playlist_path(&playlist.id), playlist.clone()))
                .and_then(|_| io::get_playlist(playlist_path(&playlist.id)))
                .err_into()
                .map(send(sender, format!("PlaylistPost {}", playlist.title)))
                .await
        }
    }
}

fn path(source_dir: String, extension: &'static str) -> impl Fn(&Uuid) -> PathBuf {
    move |uuid| source_dir.full_path(&uuid.to_string(), extension)
}

impl FileRepo {
    pub async fn new(source_dir: String) -> lipl_core::Result<FileRepo> {
        let dir = source_dir.clone();
        let (tx, rx) = mpsc::channel::<Request>(10);
        let transaction_log: PathBuf = PathBuf::from(source_dir.clone()).join(".transaction.log");

        let log = OpenOptions::new()
            .append(true)
            .create(true)
            .open(transaction_log)?;

        let (_log_join_handle, log_tx) = start_log_thread(log);

        let join_handle = tokio::spawn(async move {
            rx.map(Ok)
                .inspect_ok(move |request| {
                    if let Some(transaction) = OptionalTransaction::from(request)
                        && let Err(error) = log_tx.send(transaction)
                    {
                        tracing::error!("Error transaction logging: {error}");
                    }
                })
                .try_for_each(|request| {
                    handle_request(
                        request,
                        source_dir.clone(),
                        path(source_dir.clone(), LYRIC_EXTENSION),
                        path(source_dir.clone(), TOML_EXTENSION),
                    )
                })
                .await
                .is_ok()
        });

        let file_repo = FileRepo {
            path: dir,
            tx,
            _join_handle: Arc::new(join_handle),
        };

        Ok(file_repo.clone())
    }
}

impl LiplRepo for FileRepo {
    fn get_lyrics(&self) -> BoxFuture<'_, lipl_core::Result<Vec<Lyric>>> {
        select(self.tx.clone(), Request::LyricList)
            .err_into()
            .boxed()
    }

    fn get_lyric_summaries(&self) -> BoxFuture<'_, lipl_core::Result<Vec<Summary>>> {
        select(self.tx.clone(), Request::LyricSummaries)
            .err_into()
            .boxed()
    }

    fn get_lyric(&self, id: Uuid) -> BoxFuture<'_, lipl_core::Result<Lyric>> {
        select_by_id(self.tx.clone(), id, Request::LyricItem)
            .err_into()
            .boxed()
    }

    fn upsert_lyric(&self, lyric: Lyric) -> BoxFuture<'_, lipl_core::Result<Lyric>> {
        post(self.tx.clone(), lyric, Request::LyricPost)
            .err_into()
            .boxed()
    }

    fn delete_lyric(&self, id: Uuid) -> BoxFuture<'_, lipl_core::Result<()>> {
        delete_by_id(self.tx.clone(), id, Request::LyricDelete)
            .err_into()
            .boxed()
    }

    fn get_playlists(&self) -> BoxFuture<'_, lipl_core::Result<Vec<Playlist>>> {
        select(self.tx.clone(), Request::PlaylistList)
            .err_into()
            .boxed()
    }

    fn get_playlist_summaries(&self) -> BoxFuture<'_, lipl_core::Result<Vec<Summary>>> {
        select(self.tx.clone(), Request::PlaylistSummaries)
            .err_into()
            .boxed()
    }

    fn get_playlist(&self, id: Uuid) -> BoxFuture<'_, lipl_core::Result<Playlist>> {
        select_by_id(self.tx.clone(), id, Request::PlaylistItem)
            .err_into()
            .boxed()
    }

    fn upsert_playlist(&self, playlist: Playlist) -> BoxFuture<'_, lipl_core::Result<Playlist>> {
        post(self.tx.clone(), playlist, Request::PlaylistPost)
            .err_into()
            .boxed()
    }

    fn delete_playlist(&self, id: Uuid) -> BoxFuture<'_, lipl_core::Result<()>> {
        delete_by_id(self.tx.clone(), id, Request::PlaylistDelete)
            .err_into()
            .boxed()
    }

    fn stop(&self) -> BoxFuture<'_, lipl_core::Result<()>> {
        select(self.tx.clone(), Request::Stop).err_into().boxed()
    }
}

#[cfg(test)]
mod test {}
