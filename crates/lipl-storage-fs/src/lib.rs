use lipl_core::transaction::{start_log_thread, OptionalTransaction};
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::task::JoinHandle;

use async_trait::async_trait;

use constant::{LYRIC_EXTENSION, YAML_EXTENSION};
use fs::IO;
use futures::channel::mpsc;
use futures::{FutureExt, StreamExt, TryFutureExt, TryStreamExt};
pub use lipl_core::error::{Error, ErrorExtension};
use lipl_core::vec_ext::VecExt;
use lipl_core::{transaction::Request, LiplRepo, Lyric, Playlist, Summary, ToRepo, Uuid};
use request::{delete_by_id, post, select, select_by_id};

mod constant;
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
        s.is_dir()
            .map_err(lipl_core::Error::from)
            .map(|_| FileRepoConfig { path: s.into() })
    }
}

#[async_trait]
impl ToRepo for FileRepoConfig {
    async fn to_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        let repo = FileRepo::new(self.path).await?;
        Ok(Arc::new(repo))
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
        write!(f, "FileRepo:{}", self.path)
    }
}

fn check_members(
    playlist: &Playlist,
    lyric_ids: &[Uuid],
) -> impl futures::Future<Output = Result<(), Error>> {
    if let Some(member) = playlist
        .members
        .iter()
        .find(|member| !lyric_ids.contains(member))
    {
        futures::future::ready(Err(Error::PlaylistInvalidMember(
            playlist.id.to_string(),
            member.to_string(),
        )))
    } else {
        futures::future::ready(Ok(()))
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
                .map(|v| sender.send(v))
                .map_err(|_| lipl_core::Error::SendFailed("Stop".to_string()))
                .await?;
            Err(lipl_core::Error::Stop)
        }
        Request::LyricSummaries(sender) => {
            io::get_list(&source_dir, LYRIC_EXTENSION, io::get_lyric_summary)
                .err_into()
                .map(|v| sender.send(v))
                .map_err(|_| lipl_core::Error::SendFailed("LyricSummaries".to_string()))
                .await
        }
        Request::LyricList(sender) => {
            io::get_list(&source_dir, LYRIC_EXTENSION, io::get_lyric)
                .err_into()
                .map(|v| sender.send(v))
                .map_err(|_| lipl_core::Error::SendFailed("LyricList".to_string()))
                .await
        }
        Request::LyricItem(uuid, sender) => {
            io::get_lyric(lyric_path(&uuid))
                .err_into()
                .map(|v| sender.send(v))
                .map_err(|_| lipl_core::Error::SendFailed(format!("LyricItem {uuid}")))
                .await
        }
        Request::LyricDelete(uuid, sender) => {
            async {
                let playlists = lyric_path(&uuid)
                    .remove()
                    .and_then(|_| io::get_list(&source_dir, YAML_EXTENSION, io::get_playlist))
                    .await?;
                for mut playlist in playlists {
                    if playlist.members.contains(&uuid) {
                        playlist.members = playlist.members.without(&uuid);
                        io::post_item(
                            source_dir.full_path(&uuid.to_string(), YAML_EXTENSION),
                            playlist,
                        )
                        .await?;
                    }
                }
                Ok::<(), lipl_core::Error>(())
            }
            .map(|v| sender.send(v))
            .map_err(|_| lipl_core::Error::SendFailed(format!("LyricDelete {uuid}")))
            .await
        }
        Request::LyricPost(lyric, sender) => {
            let path = lyric_path(&lyric.id);
            io::post_item(&path, lyric)
                .and_then(|_| io::get_lyric(&path))
                .err_into()
                .map(|v| sender.send(v))
                .map_err(|e| {
                    lipl_core::Error::SendFailed(format!("LyricPost {}", e.unwrap().title))
                })
                .await
        }
        Request::PlaylistSummaries(sender) => {
            io::get_list(&source_dir, YAML_EXTENSION, io::get_playlist)
                .map_ok(lipl_core::to_summaries)
                .err_into()
                .map(|v| sender.send(v))
                .map_err(|_| lipl_core::Error::SendFailed("PlaylistSummaries".to_string()))
                .await
        }
        Request::PlaylistList(sender) => {
            io::get_list(&source_dir, YAML_EXTENSION, io::get_playlist)
                .err_into()
                .map(|v| sender.send(v))
                .map_err(|_| lipl_core::Error::SendFailed("PlaylistList".to_string()))
                .await
        }
        Request::PlaylistItem(uuid, sender) => {
            io::get_playlist(playlist_path(&uuid))
                .err_into()
                .map(|v| sender.send(v))
                .map_err(|_| lipl_core::Error::SendFailed(format!("PlaylistItem {uuid}")))
                .await
        }
        Request::PlaylistDelete(uuid, sender) => {
            let path = playlist_path(&uuid);
            path.remove()
                .err_into()
                .map(|v| sender.send(v))
                .map_err(|_| lipl_core::Error::SendFailed(format!("PlaylistDelete {uuid}")))
                .await
        }
        Request::PlaylistPost(playlist, sender) => {
            io::get_list(&source_dir, LYRIC_EXTENSION, io::get_lyric_summary)
                .map_ok(|summaries| lipl_core::ids(summaries.into_iter()))
                .and_then(|ids| check_members(&playlist, &ids))
                .and_then(|_| io::post_item(playlist_path(&playlist.id), playlist.clone()))
                .and_then(|_| io::get_playlist(playlist_path(&playlist.id)))
                .err_into()
                .map(|v| sender.send(v))
                .map_err(|e| {
                    lipl_core::Error::SendFailed(format!("PlaylistPost {}", e.unwrap().title))
                })
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

        let log = OpenOptions::new().append(true).create(true).open(transaction_log)?;

        let (_log_join_handle, log_tx) = start_log_thread(log);

        let join_handle = tokio::spawn(async move {
            rx.map(Ok)
                .inspect_ok(move |request| {
                    if let Some(transaction) = OptionalTransaction::from(request) {
                        if let Err(error) = log_tx.send(transaction) {
                            tracing::error!("Error transaction logging: {error}");
                        }
                    }
                })
                .try_for_each(|request| {
                    handle_request(
                        request,
                        source_dir.clone(),
                        path(source_dir.clone(), LYRIC_EXTENSION),
                        path(source_dir.clone(), YAML_EXTENSION),
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

        // if Path::exists(&transaction_log) {
        //     let file = OpenOptions::new().read(true).open(&transaction_log)?;
        //     build_from_log(file, file_repo.clone()).await?;
        // }

        Ok(file_repo.clone())
    }
}

#[async_trait]
impl LiplRepo for FileRepo {
    async fn get_lyrics(&self) -> lipl_core::Result<Vec<Lyric>> {
        select(self.tx.clone(), Request::LyricList).err_into().await
    }

    async fn get_lyric_summaries(&self) -> lipl_core::Result<Vec<Summary>> {
        select(self.tx.clone(), Request::LyricSummaries)
            .err_into()
            .await
    }

    async fn get_lyric(&self, id: Uuid) -> lipl_core::Result<Lyric> {
        select_by_id(self.tx.clone(), id, Request::LyricItem)
            .err_into()
            .await
    }

    async fn upsert_lyric(&self, lyric: Lyric) -> lipl_core::Result<Lyric> {
        post(self.tx.clone(), lyric, Request::LyricPost)
            .err_into()
            .await
    }

    async fn delete_lyric(&self, id: Uuid) -> lipl_core::Result<()> {
        delete_by_id(self.tx.clone(), id, Request::LyricDelete)
            .err_into()
            .await
    }

    async fn get_playlists(&self) -> lipl_core::Result<Vec<Playlist>> {
        select(self.tx.clone(), Request::PlaylistList)
            .err_into()
            .await
    }

    async fn get_playlist_summaries(&self) -> lipl_core::Result<Vec<Summary>> {
        select(self.tx.clone(), Request::PlaylistSummaries)
            .err_into()
            .await
    }

    async fn get_playlist(&self, id: Uuid) -> lipl_core::Result<Playlist> {
        select_by_id(self.tx.clone(), id, Request::PlaylistItem)
            .err_into()
            .await
    }

    async fn upsert_playlist(&self, playlist: Playlist) -> lipl_core::Result<Playlist> {
        post(self.tx.clone(), playlist, Request::PlaylistPost)
            .err_into()
            .await
    }

    async fn delete_playlist(&self, id: Uuid) -> lipl_core::Result<()> {
        delete_by_id(self.tx.clone(), id, Request::PlaylistDelete)
            .err_into()
            .await
    }

    async fn stop(&self) -> lipl_core::Result<()> {
        select(self.tx.clone(), Request::Stop).err_into().await
    }
}

#[cfg(test)]
mod test {}
