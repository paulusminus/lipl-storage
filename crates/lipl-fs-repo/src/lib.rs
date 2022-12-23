use std::fmt::Debug;
use std::str::FromStr;
// use futures::future::{Ready};
use std::path::PathBuf;
use std::sync::Arc;
// use futures::future::ready;
use tokio::task::JoinHandle;

use async_trait::async_trait;

pub use error::FileRepoError;
use fs::IO;
use futures::{channel::mpsc};
use futures::{FutureExt, StreamExt, TryStreamExt, TryFutureExt};
use lipl_core::{
    LiplRepo, Lyric, Playlist, error::{ModelError}, Summary, Uuid, ext::VecExt, into_anyhow_error,
};
use request::{delete_by_id, post, select, select_by_id, Request};
use constant::{LYRIC_EXTENSION, YAML_EXTENSION};

mod constant;
mod error;
mod fs;
mod io;
mod request;

#[derive(Clone)]
pub struct FileRepoConfig {
    pub path: String,
}

impl FromStr for FileRepoConfig {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.is_dir()
            .map_err(into_anyhow_error)
            .map(|_| FileRepoConfig { path: s.into() })
    }
}

// impl std::future::IntoFuture for FileRepoConfig {
//     type Output = anyhow::Result<FileRepo>;
//     type IntoFuture = Ready<Self::Output>;

//     fn into_future(self) -> Self::IntoFuture {
//         ready(FileRepo::new(self.path))
//     }
// }

#[derive(Clone)]
pub struct FileRepo {
    // join_handle: Arc<Pin<Box<dyn Future<Output = bool>>>>,
    tx: mpsc::Sender<Request>,
    path: String,
    _join_handle: Arc<JoinHandle<bool>>,
}

impl Debug for FileRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileRepo:{}", self.path)
    }
}

fn check_members(playlist: &Playlist, lyric_ids: &[Uuid]) -> impl futures::Future<Output = Result<(), FileRepoError>> {
    if let Some(member) = playlist.members.iter().find(|member| !lyric_ids.contains(member))
    {
        futures::future::ready(Err(FileRepoError::PlaylistInvalidMember(playlist.id.to_string(), member.to_string())))
    }
    else {
        futures::future::ready(Ok(()))
    }
}


async fn handle_request<P, Q>(request: Request, source_dir: String, lyric_path: P, playlist_path: Q) -> Result<(), ModelError> 
where P: Fn(&Uuid) -> PathBuf, Q: Fn(&Uuid) -> PathBuf
{
    match request {
        Request::Stop(sender) => {
            async {
                Ok::<(), FileRepoError>(())
            }
            .map(|v| sender.send(v))
            .map_err(|_| ModelError::SendFailed("Stop".to_string()))
            .await?;
            Err(ModelError::Stop)
        },
        Request::LyricSummaries(sender) => {
            io::get_list(
                &source_dir,
                LYRIC_EXTENSION,
                io::get_lyric_summary,
            )
            .map(|v|sender.send(v))
            .map_err(|_| ModelError::SendFailed("LyricSummaries".to_string()))
            .await
        }
        Request::LyricList(sender) => {
            io::get_list(
                &source_dir, 
                LYRIC_EXTENSION, 
                io::get_lyric,
            )
            .map(|v| sender.send(v))
            .map_err(|_| ModelError::SendFailed("LyricList".to_string()))
            .await
        }
        Request::LyricItem(uuid, sender) => {
            io::get_lyric(lyric_path(&uuid))
            .map(|v| sender.send(v))
            .map_err(|_| ModelError::SendFailed(format!("LyricItem {uuid}")))
            .await
        }
        Request::LyricDelete(uuid, sender) => {
            async {
                let playlists =
                    lyric_path(&uuid)
                    .remove()
                    .and_then(|_|
                        io::get_list(
                            &source_dir,
                            YAML_EXTENSION,
                            io::get_playlist
                        )
                    )
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
                Ok::<(), FileRepoError>(())
            }
            .map(|v| sender.send(v))
            .map_err(|_| ModelError::SendFailed(format!("LyricDelete {uuid}")))
            .await
        }
        Request::LyricPost(lyric, sender) => {
            let path = lyric_path(&lyric.id);
            io::post_item(
                &path,
                lyric,
            )
            .and_then(|_| io::get_lyric(&path))
            .map(|v| sender.send(v))
            .map_err(|e| ModelError::SendFailed(format!("LyricPost {}", e.unwrap().title)))
            .await
        }
        Request::PlaylistSummaries(sender) => {
            io::get_list(
                &source_dir,
                YAML_EXTENSION,
                io::get_playlist,
            )
            .map_ok(lipl_core::summaries)
            .map(|v| sender.send(v))
            .map_err(|_| ModelError::SendFailed("PlaylistSummaries".to_string()))
            .await
        }
        Request::PlaylistList(sender) => {
            io::get_list(
                &source_dir,
                YAML_EXTENSION,
                io::get_playlist
            )
            .map(|v| sender.send(v))
            .map_err(|_| ModelError::SendFailed("PlaylistList".to_string()))
            .await
        }
        Request::PlaylistItem(uuid, sender) => {
            io::get_playlist(playlist_path(&uuid))
            .map(|v| sender.send(v))
            .map_err(|_| ModelError::SendFailed(format!("PlaylistItem {uuid}")))
            .await
        }
        Request::PlaylistDelete(uuid, sender) => {
            let path = playlist_path(&uuid);
            path
            .remove()
            .map(|v| sender.send(v))
            .map_err(|_| ModelError::SendFailed(format!("PlaylistDelete {uuid}")))
            .await
        }
        Request::PlaylistPost(playlist, sender) => {
            io::get_list(
                &source_dir,
                LYRIC_EXTENSION,
                io::get_lyric_summary,
            )
            .map_ok(|summaries| lipl_core::ids(summaries.into_iter()))
            .and_then(|ids| check_members(&playlist, &ids))
            .and_then(
                |_| io::post_item(
                    playlist_path(&playlist.id),
                    playlist.clone(),
                )
            )
            .and_then(|_| io::get_playlist(
                    playlist_path(&playlist.id)
                )
            )
            .map(|v| sender.send(v))
            .map_err(|e| ModelError::SendFailed(format!("PlaylistPost {}", e.unwrap().title)))
            .await
        }
    }
}

impl FileRepo {
    pub fn new(
        source_dir: String,
    ) -> anyhow::Result<FileRepo> {
        let dir = source_dir.clone();
        let (tx, rx) = mpsc::channel::<Request>(10);

        let join_handle = tokio::spawn(async move {
            let lyric_path = |uuid: &Uuid| source_dir.clone().full_path(&uuid.to_string(), LYRIC_EXTENSION);
            let playlist_path = |uuid: &Uuid| source_dir.clone().full_path(&uuid.to_string(), YAML_EXTENSION);

            rx
            .map(Ok)
            .try_for_each(|request| 
                handle_request(
                    request,
                    source_dir.clone(),
                    lyric_path,
                    playlist_path,
                )
            )
            .await
            .is_ok()
        });

        Ok(
            FileRepo {
                path: dir,
                tx,
                _join_handle: Arc::new(join_handle)
            },
        )
    }

}

#[async_trait]
impl LiplRepo for FileRepo {
    async fn get_lyrics(&self) -> anyhow::Result<Vec<Lyric>> {
        select(self.tx.clone(), Request::LyricList)
        .await
        .map_err(into_anyhow_error)
    }

    async fn get_lyric_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        select(self.tx.clone(), Request::LyricSummaries)
        .await
        .map_err(into_anyhow_error)
    }

    async fn get_lyric(&self, id: Uuid) -> anyhow::Result<Lyric> {
        select_by_id(self.tx.clone(), id, Request::LyricItem)
        .await
        .map_err(into_anyhow_error)
    }

    async fn post_lyric(&self, lyric: Lyric) -> anyhow::Result<Lyric> {
        post(self.tx.clone(), lyric, Request::LyricPost)
        .await
        .map_err(into_anyhow_error)
    }

    async fn delete_lyric(&self, id: Uuid) -> anyhow::Result<()> {
        delete_by_id(self.tx.clone(), id, Request::LyricDelete)
        .await
        .map_err(into_anyhow_error)
    }

    async fn get_playlists(&self) -> anyhow::Result<Vec<Playlist>> {
        select(self.tx.clone(), Request::PlaylistList)
        .await
        .map_err(into_anyhow_error)
    }

    async fn get_playlist_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        select(self.tx.clone(), Request::PlaylistSummaries)
        .await
        .map_err(into_anyhow_error)
    }

    async fn get_playlist(&self, id: Uuid) -> anyhow::Result<Playlist> {
        select_by_id(self.tx.clone(), id, Request::PlaylistItem)
        .await
        .map_err(into_anyhow_error)
    }

    async fn post_playlist(&self, playlist: Playlist) -> anyhow::Result<Playlist> {
        post(self.tx.clone(), playlist, Request::PlaylistPost)
        .await
        .map_err(into_anyhow_error)
    }

    async fn delete_playlist(&self, id: Uuid) -> anyhow::Result<()> {
        delete_by_id(self.tx.clone(), id, Request::PlaylistDelete)
        .await
        .map_err(into_anyhow_error)
    }

    async fn stop(&self) -> anyhow::Result<()> {
        select(self.tx.clone(), Request::Stop).await
        .map_err(into_anyhow_error)
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;


    #[test]
    fn file_repo_is_sized() {
        assert_eq!(size_of::<super::FileRepo>(), 56);
    }
}