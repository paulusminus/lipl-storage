use std::sync::Arc;

use async_trait::async_trait;

use fs::IO;
use futures::channel::mpsc;
use futures::StreamExt;
use lipl_types::{
    request::{delete_by_id, post, select, select_by_id, Request},
    LiplRepo, Lyric, Playlist, RepoError, RepoResult, Summary, Uuid, Without,
};
use tokio::task::JoinHandle;

pub mod elapsed;
mod fs;
mod io;

#[derive(Clone)]
pub struct FileRepo {
    join_handle: Arc<JoinHandle<RepoResult<()>>>,
    tx: mpsc::Sender<Request>,
}

impl FileRepo {
    pub fn new(
        s: String,
        playlist_extension: String,
        lyric_extension: String,
    ) -> RepoResult<FileRepo> {
        s.is_dir()?;

        let (tx, mut rx) = mpsc::channel::<Request>(10);

        Ok(FileRepo {
            tx,
            join_handle: Arc::new(tokio::spawn(async move {
                while let Some(request) = rx.next().await {
                    match request {
                        Request::LyricSummaries(sender) => {
                            sender
                                .send(
                                    io::get_list(
                                        s.clone(),
                                        &lyric_extension,
                                        io::get_lyric_summary,
                                    )
                                    .await,
                                )
                                .map_err(|_| RepoError::SendFailed("LyricSummaries".to_owned()))?;
                        }
                        Request::LyricList(sender) => {
                            sender
                                .send(
                                    io::get_list(s.clone(), &lyric_extension, io::get_lyric).await,
                                )
                                .map_err(|_| RepoError::SendFailed("LyricList".to_owned()))?;
                        }
                        Request::LyricItem(uuid, sender) => {
                            sender
                                .send(
                                    io::get_lyric(s.full_path(&uuid.to_string(), &lyric_extension))
                                        .await,
                                )
                                .map_err(|_| RepoError::SendFailed("LyricItem".to_owned()))?;
                        }
                        Request::LyricDelete(uuid, sender) => {
                            let f = async {
                                s.full_path(&uuid.to_string(), &lyric_extension)
                                    .remove()
                                    .await?;
                                let playlists =
                                    io::get_list(s.clone(), &playlist_extension, io::get_playlist)
                                        .await?;
                                for mut playlist in playlists {
                                    if playlist.members.contains(&uuid) {
                                        playlist.members = playlist.members.without(&uuid);
                                        io::post_item(
                                            s.full_path(&uuid.to_string(), &playlist_extension),
                                            playlist,
                                        )
                                        .await?;
                                    }
                                }
                                Ok::<(), RepoError>(())
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| RepoError::SendFailed("LyricDelete".to_owned()))?;
                        }
                        Request::LyricPost(lyric, sender) => {
                            let f = async {
                                io::post_item(s.full_path(&lyric.id.to_string(), &lyric_extension), lyric.clone()).await?;
                                let lyric = io::get_lyric(s.full_path(&lyric.id.to_string(), &lyric_extension)).await?;
                                Ok::<Lyric, RepoError>(lyric)
                            };
                            sender
                                .send(
                                    f.await,
                                )
                                .map_err(|_| RepoError::SendFailed("LyricPost".to_owned()))?;
                        }
                        Request::PlaylistSummaries(sender) => {
                            sender
                                .send(
                                    io::get_list(s.clone(), &playlist_extension, io::get_playlist)
                                        .await
                                        .map(lipl_types::summaries),
                                )
                                .map_err(|_| RepoError::SendFailed("PlaylistSummaries".to_owned()))?;
                        }
                        Request::PlaylistList(sender) => {
                            sender
                                .send(
                                    io::get_list(s.clone(), &playlist_extension, io::get_playlist)
                                        .await,
                                )
                                .map_err(|_| RepoError::SendFailed("PlaylistList".to_owned()))?;
                        }
                        Request::PlaylistItem(uuid, sender) => {
                            sender
                                .send(
                                    io::get_playlist(
                                        s.full_path(&uuid.to_string(), &playlist_extension),
                                    )
                                    .await,
                                )
                                .map_err(|_| RepoError::SendFailed(format!("PlaylistItem {uuid}")))?;
                        }
                        Request::PlaylistDelete(uuid, sender) => {
                            sender
                                .send(
                                    s.full_path(&uuid.to_string(), &playlist_extension)
                                        .remove()
                                        .await,
                                )
                                .map_err(|_| RepoError::SendFailed("PlaylistDelete".to_owned()))?;
                        }
                        Request::PlaylistPost(playlist, sender) => {
                            let f = async {
                                let summaries = io::get_list(
                                    s.clone(),
                                    &lyric_extension,
                                    io::get_lyric_summary,
                                )
                                .await?;
                                let lyric_ids = lipl_types::ids(summaries.into_iter());
                                for member in playlist.members.iter() {
                                    if !lyric_ids.contains(member) {
                                        return Err(RepoError::PlaylistInvalidMember(
                                            playlist.id.to_string(),
                                            member.to_string(),
                                        ));
                                    }
                                }
                                let playlist = io::get_playlist(s.full_path(&playlist.id.to_string(), &playlist_extension)).await?;
                                Ok::<Playlist, RepoError>(playlist)
                            };
                            sender
                                .send(f.await)
                                .map_err(|_| RepoError::SendFailed("PlaylistPost".to_owned()))?;
                        }
                    }
                }

                Ok::<(), RepoError>(())
            })),
        })
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
