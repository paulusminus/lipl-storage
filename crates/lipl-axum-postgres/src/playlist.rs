use async_trait::async_trait;
use futures_util::TryFutureExt;
use lipl_core::{ext::VecExt, Playlist, PlaylistDb, PlaylistPost, Summary, Uuid};

use super::convert;
use crate::{PostgresConnectionPool};

#[async_trait]
impl PlaylistDb for PostgresConnectionPool {
    async fn playlist_list(&self) -> lipl_core::Result<Vec<Summary>> {
        self.query(sql::LIST, sql::LIST_TYPES, convert::to_summary, &[])
        .err_into()
        .await
    }

    async fn playlist_list_full(&self) -> lipl_core::Result<Vec<Playlist>> {
        self.query(sql::LIST_FULL, sql::LIST_FULL_TYPES, convert::to_playlist, &[])
        .err_into()
        .await
    }

    async fn playlist_item(&self, uuid: Uuid) -> lipl_core::Result<Playlist> {
        self.query_one(sql::ITEM, sql::ITEM_TYPES, convert::to_playlist, &[&uuid.inner()])
        .err_into()
        .await
    }

    async fn playlist_delete(&self, uuid: Uuid) -> lipl_core::Result<()> {
        self.execute(sql::DELETE, sql::DELETE_TYPES, &[&uuid.inner()])
        .err_into()
        .await
    }

    async fn playlist_post(&self, playlist_post: PlaylistPost) -> lipl_core::Result<Playlist> {
        self.query_one(
            sql::UPSERT,
            sql::UPSERT_TYPES,
            convert::to_playlist,
            &[
                &Uuid::default().inner(),
                &playlist_post.title.clone(),
                &playlist_post.members.map(convert::to_inner).as_slice()
            ])
            .err_into()
            .await
    }

    async fn playlist_put(
        &self,
        uuid: Uuid,
        playlist_post: PlaylistPost,
    ) -> lipl_core::Result<Playlist> {
        self.query_one(
            sql::UPSERT,
            sql::UPSERT_TYPES,
            convert::to_playlist,
            &[
                &uuid.inner(),
                &playlist_post.title.clone(),
                &playlist_post.members.map(convert::to_inner).as_slice()
            ])
            .err_into()
            .await
    }
}

mod sql {
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
