use std::sync::Arc;

use async_trait::async_trait;

use fs::IO;
use futures::{channel::mpsc};
use futures::StreamExt;
use lipl_types::{
    LiplRepo, Lyric, Playlist, RepoError, RepoResult, Summary, Uuid, Without,
};
use request::{delete_by_id, post, select, select_by_id, Request};
use constant::{LYRIC_EXTENSION, YAML_EXTENSION};

use tokio::task::JoinHandle;

mod constant;
pub mod elapsed;
mod fs;
mod io;
mod request;

#[derive(Clone)]
pub struct FileRepo {
    join_handle: Arc<JoinHandle<RepoResult<()>>>,
    tx: mpsc::Sender<Request>,
}

impl FileRepo {
    pub fn new(
        source_dir: String,
    ) -> RepoResult<FileRepo> {
        source_dir.is_dir()?;

        let (tx, mut rx) = mpsc::channel::<Request>(10);

        Ok(FileRepo {
            tx,
            join_handle: Arc::new(tokio::spawn(async move {
                let lyric_path = |uuid: &Uuid| source_dir.full_path(&uuid.to_string(), LYRIC_EXTENSION);
                let playlist_path = |uuid: &Uuid| source_dir.full_path(&uuid.to_string(), YAML_EXTENSION);
                while let Some(request) = rx.next().await {
                    match request {
                        Request::Stop(sender) => {
                            sender.send(Ok(()))
                            .map_err(|_| RepoError::SendFailed("Stop".to_string()))?;
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
                                .map_err(|_| RepoError::SendFailed("LyricSummaries".to_string()))?;
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
                                .map_err(|_| RepoError::SendFailed("LyricList".to_string()))?;
                        }
                        Request::LyricItem(uuid, sender) => {
                            let f = async {
                                io::get_lyric(lyric_path(&uuid))
                                .await
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| RepoError::SendFailed(format!("LyricItem {uuid}")))?;
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
                                Ok::<(), RepoError>(())
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| RepoError::SendFailed(format!("LyricDelete {uuid}")))?;
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
                                Ok::<Lyric, RepoError>(lyric)
                            };
                            sender
                                .send(
                                    f.await,
                                )
                                .map_err(|e| RepoError::SendFailed(format!("LyricPost {}", e.unwrap().title)))?;
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
                                .map_err(|_| RepoError::SendFailed("PlaylistSummaries".to_string()))?;
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
                                .map_err(|_| RepoError::SendFailed("PlaylistList".to_string()))?;
                        }
                        Request::PlaylistItem(uuid, sender) => {
                            let f = async {
                                io::get_playlist(playlist_path(&uuid))
                                .await
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| RepoError::SendFailed(format!("PlaylistItem {uuid}")))?;
                        }
                        Request::PlaylistDelete(uuid, sender) => {
                            let f = async {
                                playlist_path(&uuid)
                                .remove()
                                .await
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| RepoError::SendFailed(format!("PlaylistDelete {uuid}")))?;
                        }
                        Request::PlaylistPost(playlist, sender) => {
                            let f = async {
                                let id = playlist.id.clone();
                                let summaries = io::get_list(
                                    &source_dir,
                                    LYRIC_EXTENSION,
                                    io::get_lyric_summary,
                                )
                                .await?;
                                let lyric_ids = lipl_types::ids(summaries.into_iter());
                                for member in playlist.members.iter() {
                                    if !lyric_ids.contains(member) {
                                        return Err(RepoError::PlaylistInvalidMember(
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
                                Ok::<Playlist, RepoError>(playlist)
                            };
                            sender
                                .send(f.await)
                                .map_err(|e| RepoError::SendFailed(format!("PlaylistPost {}", e.unwrap().title)))?;
                        }
                    }
                }

                Ok::<(), RepoError>(())
            })),
        })
    }

    pub async fn stop(&self) -> RepoResult<()> {
        select(&mut self.tx.clone(), Request::Stop).await?;
        self.join_handle.abort();
        Ok(())
    }
}

#[async_trait]
impl LiplRepo for FileRepo {
    async fn get_lyrics(&self) -> RepoResult<Vec<Lyric>> {
        select(&mut self.tx.clone(), Request::LyricList).await
    }

    async fn get_lyric_summaries(&self) -> RepoResult<Vec<Summary>> {
        select(&mut self.tx.clone(), Request::LyricSummaries).await
    }

    async fn get_lyric(&self, id: Uuid) -> RepoResult<Lyric> {
        select_by_id(&mut self.tx.clone(), id, Request::LyricItem).await
    }

    async fn post_lyric(&self, lyric: Lyric) -> RepoResult<Lyric> {
        post(&mut self.tx.clone(), lyric, Request::LyricPost).await
    }

    async fn delete_lyric(&self, id: Uuid) -> RepoResult<()> {
        delete_by_id(&mut self.tx.clone(), id, Request::LyricDelete).await
    }

    async fn get_playlists(&self) -> RepoResult<Vec<Playlist>> {
        select(&mut self.tx.clone(), Request::PlaylistList).await
    }

    async fn get_playlist_summaries(&self) -> RepoResult<Vec<Summary>> {
        select(&mut self.tx.clone(), Request::PlaylistSummaries).await
    }

    async fn get_playlist(&self, id: Uuid) -> RepoResult<Playlist> {
        select_by_id(&mut self.tx.clone(), id, Request::PlaylistItem).await
    }

    async fn post_playlist(&self, playlist: Playlist) -> RepoResult<Playlist> {
        post(&mut self.tx.clone(), playlist, Request::PlaylistPost).await
    }

    async fn delete_playlist(&self, id: Uuid) -> RepoResult<()> {
        delete_by_id(&mut self.tx.clone(), id, Request::PlaylistDelete).await
    }
}
