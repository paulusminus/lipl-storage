use std::path::PathBuf;

use async_trait::async_trait;

use fs::IO;
use futures::StreamExt;
use futures::channel::mpsc;
use lipl_types::{
    LiplRepo,
    Lyric,
    Playlist,
    PlaylistPost,
    RepoResult,
    Summary,
    Without,
    Uuid,
    RepoError,
    request::Request,
};
use tokio::task::JoinHandle;

pub mod elapsed;
mod fs;
mod io;

#[derive(Clone)]
pub struct FileSystem {
    source_dir: String,
    playlist_extension: String,
    lyric_extension: String,
}

impl FileSystem {
    pub fn new(s: String, playlist_extension: String, lyric_extension: String) -> RepoResult<(mpsc::Sender<Request>, JoinHandle<Result<(), RepoError>>)> {
        s.is_dir()?;


        let (tx, mut rx) = mpsc::channel::<Request>(10);

        let filesystem = FileSystem {
            source_dir: s,
            playlist_extension,
            lyric_extension,
        };

        let joiner = tokio::spawn(async move {
            while let Some(request) = rx.next().await {
                match request {
                    Request::LyricSummaries(sender) => {
                        sender.send(
                            filesystem.get_lyric_summaries().await
                        )
                        .map_err(|_| RepoError::SendFailed("LyricSummaries"))?;
                    },
                    Request::LyricList(sender) => {
                        sender.send(
                            filesystem.get_lyrics().await
                        )
                        .map_err(|_| RepoError::SendFailed("LyricList"))?;
                    },
                    Request::LyricItem(uuid, sender) => {
                        sender.send(
                            filesystem.get_lyric(uuid)
                            .await
                        )
                        .map_err(|_| RepoError::SendFailed("LyricItem"))?;
                    },
                    Request::LyricDelete(uuid, sender) => {
                            sender.send(
                                filesystem.delete_lyric(uuid).await
                            )
                            .map_err(|_| RepoError::SendFailed("LyricDelete"))?;
                    },
                    Request::LyricPost(lyric_post, sender) => {
                        let lyric: Lyric = (None, lyric_post).into();
                        sender.send(
                            filesystem.post_lyric(lyric).await
                        )
                        .map_err(|_| RepoError::SendFailed("LyricPost"))?;

                    },
                    Request::LyricPut(uuid, lyric_post, sender) => {
                        let lyric: Lyric = (Some(uuid), lyric_post).into();
                        sender.send(
                            filesystem.post_lyric(lyric).await
                        )
                        .map_err(|_| RepoError::SendFailed("LyricPut"))?;
                    },
                    Request::PlaylistSummaries(sender) => {
                        sender.send(
                            filesystem.get_playlist_summaries().await
                        )
                        .map_err(|_| RepoError::SendFailed("PlaylistSummaries"))?;
                    },
                    Request::PlaylistList(sender) => {
                        sender.send(
                            filesystem.get_playlists().await
                        )
                        .map_err(|_| RepoError::SendFailed("PlaylistList"))?;
                    },
                    Request::PlaylistItem(uuid, sender) => {
                        sender.send(
                            filesystem.get_playlist(uuid).await
                        )
                        .map_err(|_| RepoError::SendFailed("PlaylistItem"))?;
                    },
                    Request::PlaylistDelete(uuid, sender) => {
                        sender.send(
                            filesystem.delete_playlist(uuid).await
                        )
                        .map_err(|_| RepoError::SendFailed("PlaylistDelete"))?;

                    },
                    Request::PlaylistPost(playlist_post, sender) => {
                        let playlist: Playlist = (None, playlist_post).into();
                        sender.send(
                            filesystem.post_playlist(playlist).await
                        )
                        .map_err(|_| RepoError::SendFailed("PlaylistPost"))?;
                    },
                    Request::PlaylistPut(uuid, playlist_post, sender) => {
                        let playlist: Playlist = (Some(uuid), playlist_post).into();
                        sender.send(
                            filesystem.post_playlist(playlist).await
                        )
                        .map_err(|_| RepoError::SendFailed("PlaylistPut"))?;
                    },
                }
            }

            Ok::<(), RepoError>(())
        });

        Ok((tx, joiner))
    }

    fn playlist_path(&self, id: &Uuid) -> PathBuf {
        self.source_dir.full_path(&id.to_string(), &self.playlist_extension)
    }

    fn lyric_path(&self, id: &Uuid) -> PathBuf {
        self.source_dir.full_path(&id.to_string(), &self.lyric_extension)
    }
}

#[async_trait]
impl LiplRepo for FileSystem {
    async fn get_lyrics(&self) -> RepoResult<Vec<Lyric>> {
        // self.lock.read().await;
        io::get_list(
            &self.source_dir,
            &self.lyric_extension,
            io::get_lyric,
        )
        .await
    }

    async fn get_lyric_summaries(&self) -> RepoResult<Vec<Summary>> {
        // self.lock.read().await;
        io::get_list(
            &self.source_dir,
            &self.lyric_extension,
            io::get_lyric_summary,
        )
        .await
    }

    async fn get_lyric(&self, id: Uuid) -> RepoResult<Lyric> {
        // self.lock.read().await;
        io::get_lyric(
            self.lyric_path(&id)
        )
        .await
    }

    async fn post_lyric(&self, lyric: Lyric) -> RepoResult<()> {
        // self.lock.write().await;
        io::post_item(
            self.lyric_path(&lyric.id),
            lyric,
        )
        .await
    }

    async fn delete_lyric(&self, id: Uuid) -> RepoResult<()> {
        // self.lock.write().await;
        self.lyric_path(&id).remove().await?;
        for mut playlist in self.get_playlists().await? {
            if playlist.members.contains(&id) {
                playlist.members = playlist.members.without(&id);
                self.post_playlist(playlist).await?;
            }
        }
        Ok(())
    }

    async fn get_playlists(&self) -> RepoResult<Vec<Playlist>> {
        // self.lock.read().await;
        io::get_list(&self.source_dir, &self.playlist_extension, io::get_playlist).await
    }

    async fn get_playlist_summaries(&self) -> RepoResult<Vec<Summary>> {
        // self.lock.read().await;
        self.get_playlists()
        .await
        .map(lipl_types::summaries)
    }

    async fn get_playlist(&self, id: Uuid) -> RepoResult<Playlist> {
        // self.lock.read().await;
        io::get_playlist(
            self.playlist_path(&id),
        )
        .await
    }

    async fn post_playlist(&self, playlist: Playlist) -> RepoResult<()> {
        // self.lock.write().await;
        let lyric_ids: Vec<Uuid> = lipl_types::ids(self.get_lyric_summaries().await?.into_iter());
        for member in playlist.members.iter() {
            if !lyric_ids.contains(member) {
                return Err(RepoError::PlaylistInvalidMember(playlist.id.to_string(), member.to_string()));
            }
        }
        io::post_item(
            self.playlist_path(&playlist.id),
            PlaylistPost::from(playlist),
        ).await
    }

    async fn delete_playlist(&self, id: Uuid) -> RepoResult<()> {
        // self.lock.write().await;
        self.playlist_path(&id).remove().await
    }
}
