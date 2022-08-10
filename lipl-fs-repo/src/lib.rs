use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;

pub use error::FileRepoError;
use fs::IO;
use futures::{channel::mpsc};
use futures::StreamExt;
use lipl_types::{
    time_it, LiplRepo, Lyric, Playlist, error::{ModelError, ModelResult}, Summary, Uuid, Without,
};
use request::{delete_by_id, post, select, select_by_id, Request};
use constant::{LYRIC_EXTENSION, YAML_EXTENSION};

use tokio::task::JoinHandle;

mod constant;
mod error;
mod fs;
mod io;
mod request;

#[derive(Clone)]
pub struct FileRepo {
    join_handle: Arc<JoinHandle<ModelResult<()>>>,
    tx: mpsc::Sender<Request>,
    path: String,
}

impl Debug for FileRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileRepo: {}", self.path)
    }
}

impl FileRepo {
    pub fn new(
        source_dir: String,
    ) -> ModelResult<FileRepo> {
        source_dir.is_dir().map_err(|_| ModelError::NoPath(source_dir.clone().into()))?;

        let (tx, mut rx) = mpsc::channel::<Request>(10);

        Ok(FileRepo {
            path: source_dir.clone(),
            tx,
            join_handle: Arc::new(tokio::spawn(async move {
                let lyric_path = |uuid: &Uuid| source_dir.full_path(&uuid.to_string(), LYRIC_EXTENSION);
                let playlist_path = |uuid: &Uuid| source_dir.full_path(&uuid.to_string(), YAML_EXTENSION);
                while let Some(request) = rx.next().await {
                    match request {
                        Request::Stop(sender) => {
                            sender.send(Ok(()))
                            .map_err(|_| ModelError::SendFailed("Stop".to_string()))?;
                            break;
                        },
                        Request::LyricSummaries(sender) => {
                            let f = async {
                                io::get_list(
                                    &source_dir,
                                    LYRIC_EXTENSION,
                                    io::get_lyric_summary,
                                )
                                .await
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| ModelError::SendFailed("LyricSummaries".to_string()))?;
                        }
                        Request::LyricList(sender) => {
                            let f = async {
                                io::get_list(
                                    &source_dir, 
                                    LYRIC_EXTENSION, 
                                    io::get_lyric,
                                )
                                .await
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| ModelError::SendFailed("LyricList".to_string()))?;
                        }
                        Request::LyricItem(uuid, sender) => {
                            let f = async {
                                io::get_lyric(lyric_path(&uuid))
                                .await
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| ModelError::SendFailed(format!("LyricItem {uuid}")))?;
                        }
                        Request::LyricDelete(uuid, sender) => {
                            let f = async {
                                lyric_path(&uuid)
                                    .remove()
                                    .await?;
                                let playlists =
                                    io::get_list(
                                        &source_dir,
                                        YAML_EXTENSION,
                                        io::get_playlist
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
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| ModelError::SendFailed(format!("LyricDelete {uuid}")))?;
                        }
                        Request::LyricPost(lyric, sender) => {
                            let f = async {
                                let path = lyric_path(&lyric.id);
                                io::post_item(
                                    &path,
                                    lyric,
                                )
                                .await?;
                                let lyric = io::get_lyric(&path).await?;
                                Ok::<Lyric, FileRepoError>(lyric)
                            };
                            sender
                                .send(
                                    f.await,
                                )
                                .map_err(|e| ModelError::SendFailed(format!("LyricPost {}", e.unwrap().title)))?;
                        }
                        Request::PlaylistSummaries(sender) => {
                            let f = async {
                                io::get_list(
                                    &source_dir,
                                    YAML_EXTENSION,
                                    io::get_playlist,
                                )
                                .await
                                .map(lipl_types::summaries)
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| ModelError::SendFailed("PlaylistSummaries".to_string()))?;
                        }
                        Request::PlaylistList(sender) => {
                            let f = async {
                                io::get_list(
                                    &source_dir,
                                    YAML_EXTENSION,
                                    io::get_playlist
                                )
                                .await
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| ModelError::SendFailed("PlaylistList".to_string()))?;
                        }
                        Request::PlaylistItem(uuid, sender) => {
                            let f = async {
                                io::get_playlist(playlist_path(&uuid))
                                .await
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| ModelError::SendFailed(format!("PlaylistItem {uuid}")))?;
                        }
                        Request::PlaylistDelete(uuid, sender) => {
                            let f = async {
                                playlist_path(&uuid)
                                .remove()
                                .await
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| ModelError::SendFailed(format!("PlaylistDelete {uuid}")))?;
                        }
                        Request::PlaylistPost(playlist, sender) => {
                            let f = async {
                                let id = playlist.id;
                                let summaries = io::get_list(
                                    &source_dir,
                                    LYRIC_EXTENSION,
                                    io::get_lyric_summary,
                                )
                                .await?;
                                let lyric_ids = lipl_types::ids(summaries.into_iter());
                                for member in playlist.members.iter() {
                                    if !lyric_ids.contains(member) {
                                        return Err(FileRepoError::PlaylistInvalidMember(
                                            id.to_string(),
                                            member.to_string(),
                                        ));
                                    }
                                }
                                io::post_item(
                                    playlist_path(&id),
                                    playlist
                                )
                                .await?;
                                let playlist = io::get_playlist(playlist_path(&id)).await?;
                                Ok::<Playlist, FileRepoError>(playlist)
                            };
                            sender
                                .send(f.await)
                                .map_err(|e| ModelError::SendFailed(format!("PlaylistPost {}", e.unwrap().title)))?;
                        }
                    }
                }

                Ok::<(), ModelError>(())
            })),
        })
    }

}

#[async_trait]
impl LiplRepo for FileRepo {

    #[tracing::instrument]
    async fn get_lyrics(&self) -> anyhow::Result<Vec<Lyric>> {
        time_it!(
            select(&mut self.tx.clone(), Request::LyricList)    
        )
    }

    #[tracing::instrument]
    async fn get_lyric_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        time_it!(
            select(&mut self.tx.clone(), Request::LyricSummaries)
        )
    }

    #[tracing::instrument]
    async fn get_lyric(&self, id: Uuid) -> anyhow::Result<Lyric> {
        time_it!(
            select_by_id(&mut self.tx.clone(), id, Request::LyricItem)
        )
    }

    #[tracing::instrument]
    async fn post_lyric(&self, lyric: Lyric) -> anyhow::Result<Lyric> {
        time_it!(
            post(&mut self.tx.clone(), lyric, Request::LyricPost)
        )
    }

    #[tracing::instrument]
    async fn delete_lyric(&self, id: Uuid) -> anyhow::Result<()> {
        time_it!(
            delete_by_id(&mut self.tx.clone(), id, Request::LyricDelete)
        )
    }

    #[tracing::instrument]
    async fn get_playlists(&self) -> anyhow::Result<Vec<Playlist>> {
        time_it!(
            select(&mut self.tx.clone(), Request::PlaylistList)
        )
    }

    #[tracing::instrument]
    async fn get_playlist_summaries(&self) -> anyhow::Result<Vec<Summary>> {
        time_it!(
            select(&mut self.tx.clone(), Request::PlaylistSummaries)
        )
    }

    #[tracing::instrument]
    async fn get_playlist(&self, id: Uuid) -> anyhow::Result<Playlist> {
        time_it!(
            select_by_id(&mut self.tx.clone(), id, Request::PlaylistItem)
        )
    }

    #[tracing::instrument]
    async fn post_playlist(&self, playlist: Playlist) -> anyhow::Result<Playlist> {
        time_it!(
            post(&mut self.tx.clone(), playlist, Request::PlaylistPost)
        )
    }

    #[tracing::instrument]
    async fn delete_playlist(&self, id: Uuid) -> anyhow::Result<()> {
        time_it!(
            delete_by_id(&mut self.tx.clone(), id, Request::PlaylistDelete)
        )
    }

    #[tracing::instrument]
    async fn stop(&self) -> anyhow::Result<()> {
        time_it!(
            select(&mut self.tx.clone(), Request::Stop)
        )?;
        self.join_handle.abort();
        anyhow::Ok(())
    }
}
