use async_trait::async_trait;
use futures_util::TryFutureExt;
use lipl_core::{ext::VecExt, LiplRepo, Lyric, Summary, Uuid, Playlist};
use parts::to_text;

use super::convert;
use crate::PostgresConnectionPool;

#[async_trait]
impl LiplRepo for PostgresConnectionPool {
    async fn get_lyric_summaries(&self) -> lipl_core::Result<Vec<Summary>> {
        self.query(lyric::LIST, lyric::LIST_TYPES, convert::to_summary, &[])
        .err_into()
        .await
    }

    async fn get_lyrics(&self) -> lipl_core::Result<Vec<Lyric>> {
        self.query(lyric::LIST_FULL, lyric::LIST_FULL_TYPES, convert::to_lyric, &[])
        .err_into()
        .await
    }

    async fn get_lyric(&self, uuid: Uuid) -> lipl_core::Result<Lyric> {
        self.query_one(lyric::ITEM, lyric::ITEM_TYPES, convert::to_lyric, &[&uuid.inner()])
        .err_into()
        .await
    }

    async fn upsert_lyric(&self, lyric: Lyric) -> lipl_core::Result<Lyric> {
        self.query_one(
            lyric::UPSERT,
            lyric::UPSERT_TYPES,
            convert::to_lyric,
            &[&Uuid::default().inner(), &lyric.title.clone(), &to_text(&lyric.parts)],
        )
        .err_into()
        .await
    }

    async fn delete_lyric(&self, uuid: Uuid) -> lipl_core::Result<()> {
        self.execute(lyric::DELETE, lyric::DELETE_TYPES, &[&uuid.inner()])
        .err_into()
        .await
    }

    async fn get_playlist_summaries(&self) -> lipl_core::Result<Vec<Summary>> {
        self.query(playlist::LIST, playlist::LIST_TYPES, convert::to_summary, &[])
        .err_into()
        .await
    }

    async fn get_playlists(&self) -> lipl_core::Result<Vec<Playlist>> {
        self.query(playlist::LIST_FULL, playlist::LIST_FULL_TYPES, convert::to_playlist, &[])
        .err_into()
        .await
    }

    async fn get_playlist(&self, uuid: Uuid) -> lipl_core::Result<Playlist> {
        self.query_one(playlist::ITEM, playlist::ITEM_TYPES, convert::to_playlist, &[&uuid.inner()])
        .err_into()
        .await
    }

    async fn delete_playlist(&self, uuid: Uuid) -> lipl_core::Result<()> {
        self.execute(playlist::DELETE, playlist::DELETE_TYPES, &[&uuid.inner()])
        .err_into()
        .await
    }

    async fn upsert_playlist(&self, playlist: Playlist) -> lipl_core::Result<Playlist> {
        self.query_one(
            playlist::UPSERT,
            playlist::UPSERT_TYPES,
            convert::to_playlist,
            &[
                &Uuid::default().inner(),
                &playlist.title.clone(),
                &playlist.members.map(convert::to_inner).as_slice()
            ])
            .err_into()
            .await
    }

    async fn stop(&self) -> lipl_core::Result<()> {
        Ok(())
    }
}

mod lyric {
    use tokio_postgres::types::Type;

    pub const LIST: &str = "SELECT id, title FROM lyric ORDER BY title;";
    pub const LIST_TYPES: &[Type] = &[];

    pub const LIST_FULL: &str = "SELECT id, title, parts FROM lyric ORDER BY title;";
    pub const LIST_FULL_TYPES: &[Type] = &[];

    pub const ITEM: &str = "SELECT * FROM lyric WHERE id = $1;";
    pub const ITEM_TYPES: &[Type] = &[Type::UUID];

    pub const DELETE: &str = "DELETE FROM lyric WHERE id = $1;";
    pub const DELETE_TYPES: &[Type] = &[Type::UUID];

    pub const UPSERT: &str = "SELECT * from fn_upsert_lyric($1, $2, $3)";
    pub const UPSERT_TYPES: &[Type] = &[Type::UUID, Type::VARCHAR, Type::VARCHAR];
}

mod playlist {
    use tokio_postgres::types::Type;

    pub const LIST: &str = "SELECT id, title FROM playlist ORDER BY title;";
    pub const LIST_TYPES: &[Type] = &[];

    pub const LIST_FULL: &str = "SELECT playlist.id AS id, title, ARRAY_AGG(lyric_id ORDER BY ordering) members FROM playlist INNER JOIN member ON playlist.id = playlist_id GROUP BY playlist.id ORDER BY playlist.title;";
    pub const LIST_FULL_TYPES: &[Type] = &[];

    pub const ITEM: &str = "SELECT playlist.id AS id, title, ARRAY_AGG(lyric_id ORDER BY ordering) members FROM playlist INNER JOIN member ON playlist.id = playlist_id GROUP BY playlist.id HAVING playlist.id = $1";
    pub const ITEM_TYPES: &[Type] = &[Type::UUID];

    pub const DELETE: &str = "DELETE FROM playlist WHERE id = $1;";
    pub const DELETE_TYPES: &[Type] = &[Type::UUID];

    pub const UPSERT: &str = "SELECT * from fn_upsert_playlist($1, $2, $3);";
    pub const UPSERT_TYPES: &[Type] = &[Type::UUID, Type::VARCHAR, Type::UUID_ARRAY];
}
