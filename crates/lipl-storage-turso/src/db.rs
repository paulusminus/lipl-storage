use futures_util::{TryFutureExt, TryStreamExt};
use lipl_core::postgres_error;
use lipl_core::{Error, Lyric, Playlist, Repo, Result, Summary, Uuid, parts::to_text};
use turso::Value;

use crate::TursoDatabase;

use super::convert;

fn error_on_count(count: u64, uuid: Uuid) -> Result<()> {
    if count < 1 {
        Err(Error::NoKey(uuid.to_string()))
    } else {
        Ok(())
    }
}

fn pg_error_to_lipl_core(uuid: Uuid) -> impl Fn(Error) -> lipl_core::Error {
    move |pg_error| match pg_error {
        Error::NoResults => Error::NoKey(uuid.to_string()),
        _ => pg_error,
    }
}

impl Repo for TursoDatabase {
    async fn get_lyric_summaries(&self) -> Result<Vec<Summary>> {
        self.query(lyric::LIST, convert::to_summary, Vec::<&str>::new())
            .map_err(postgres_error)
            .await?
            .try_collect::<Vec<Summary>>()
            .await
    }

    async fn get_lyrics(&self) -> Result<Vec<Lyric>> {
        self.query(lyric::LIST_FULL, convert::to_lyric, Vec::<&str>::new())
            .map_err(postgres_error)
            .await?
            .try_collect::<Vec<Lyric>>()
            .await
    }

    async fn get_lyric(&self, uuid: Uuid) -> Result<Lyric> {
        self.query_one(lyric::ITEM, convert::to_lyric, &[uuid.to_string().as_str()])
            .map_err(pg_error_to_lipl_core(uuid))
            .await
    }

    async fn upsert_lyric(&self, lyric: Lyric) -> Result<Lyric> {
        self.query_one(
            lyric::UPSERT,
            convert::to_lyric,
            &[
                lyric.id.to_string().as_str(),
                lyric.title.as_str(),
                to_text(&lyric.parts).as_str(),
            ],
        )
        .err_into()
        .await
    }

    async fn delete_lyric(&self, uuid: Uuid) -> Result<()> {
        let count = self
            .execute(lyric::DELETE, &[uuid.to_string().as_str()])
            .await?;
        error_on_count(count, uuid)
    }

    async fn get_playlist_summaries(&self) -> Result<Vec<Summary>> {
        self.query(playlist::LIST, convert::to_summary, Vec::<&str>::new())
            .map_err(postgres_error)
            .await?
            .try_collect::<Vec<Summary>>()
            .await
    }

    async fn get_playlists(&self) -> Result<Vec<Playlist>> {
        use futures_util::stream::TryStreamExt;
        self.query(
            playlist::LIST_FULL,
            convert::to_playlist,
            Vec::<&str>::new(),
        )
        .await?
        .try_collect::<Vec<_>>()
        .await
    }

    async fn get_playlist(&self, uuid: Uuid) -> Result<Playlist> {
        let playlist = self
            .query_one(
                playlist::ITEM,
                convert::to_playlist,
                &[uuid.to_string().as_str()],
            )
            .map_err(pg_error_to_lipl_core(uuid))
            .await?;
        Ok(playlist)
    }

    async fn delete_playlist(&self, uuid: Uuid) -> Result<()> {
        let count = self
            .execute(playlist::DELETE, &[uuid.to_string().as_str()])
            .await?;
        error_on_count(count, uuid)
    }

    async fn upsert_playlist(&self, playlist: Playlist) -> Result<Playlist> {
        let mut connection = self.inner.clone();
        let transaction = connection.transaction().await.map_err(postgres_error)?;
        dbg!("Starting upserting playlist");
        let _ = transaction
            .execute(
                playlist::UPSERT,
                &[playlist.id.to_string().as_str(), playlist.title.as_str()],
            )
            .await
            .map_err(postgres_error)?;
        dbg!("Finished upserting playlist");
        dbg!("Starting deleting members");
        let _ = transaction
            .execute(member::DELETE, &[playlist.id.to_string().as_str()])
            .await
            .map_err(postgres_error)?;
        dbg!("Finished deleting members");
        dbg!("Starting inserting members");
        for (index, lyric_id) in playlist.members.iter().enumerate() {
            let _ = transaction
                .execute(
                    member::INSERT,
                    &[
                        Value::from(playlist.id.to_string().as_str()),
                        Value::from(lyric_id.to_string().as_str()),
                        Value::from(index as i64),
                    ],
                )
                .await
                .map_err(postgres_error)?;
        }
        dbg!("Finished inserting members");
        transaction.commit().await.map_err(postgres_error)?;
        Ok(playlist)
    }

    async fn stop(&self) -> Result<()> {
        Ok(())
    }
}

mod lyric {
    pub const LIST: &str = "SELECT id, title FROM lyric ORDER BY title;";
    pub const LIST_FULL: &str = "SELECT id, title, parts FROM lyric ORDER BY title;";
    pub const ITEM: &str = "SELECT * FROM lyric WHERE id = $1;";
    pub const DELETE: &str = "DELETE FROM lyric WHERE id = $1;";
    pub const UPSERT: &str = "INSERT INTO lyric (id, title, parts) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET title = $2, parts = $3 RETURNING id, title, parts;";
}

mod playlist {
    pub const LIST: &str = "SELECT id, title FROM playlist ORDER BY title;";
    pub const LIST_FULL: &str = "SELECT playlist.id AS id, title, GROUP_CONCAT(lyric_id) members FROM playlist LEFT JOIN (SELECT * FROM member ORDER BY ordering) ON playlist.id = playlist_id GROUP BY playlist.id ORDER BY playlist.title;";
    pub const ITEM: &str = "SELECT playlist.id AS id, title, GROUP_CONCAT(lyric_id) members FROM playlist LEFT JOIN (SELECT * FROM member ORDER BY ordering) ON playlist.id = playlist_id GROUP BY playlist.id HAVING playlist.id = $1;";
    pub const DELETE: &str = "DELETE FROM playlist WHERE id = $1;";
    pub const UPSERT: &str = "INSERT INTO playlist (id, title) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE SET title = $2;";
}

mod member {
    pub const DELETE: &str = "DELETE FROM member WHERE playlist_id = $1;";
    pub const INSERT: &str =
        "INSERT INTO member (playlist_id, lyric_id, ordering) VALUES ($1, $2, $3);";
}
