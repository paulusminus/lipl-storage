use async_trait::async_trait;
use lipl_core::{ext::VecExt, Playlist, PlaylistDb, PlaylistPost, Summary, Uuid};

use super::convert;
use crate::{error::Error, PostgresConnectionPool};

#[async_trait]
impl PlaylistDb for PostgresConnectionPool {
    type Error = Error;
    async fn playlist_list(&self) -> Result<Vec<Summary>, Self::Error> {
        self.query(sql::LIST, sql::LIST_TYPES, convert::to_summary, &[]).await
    }

    async fn playlist_item(&self, uuid: Uuid) -> Result<Playlist, Self::Error> {
        self.query_one(sql::ITEM, sql::ITEM_TYPES, convert::to_playlist, &[&uuid.inner()]).await
    }

    async fn playlist_delete(&self, uuid: Uuid) -> Result<(), Self::Error> {
        self.execute(sql::DELETE, sql::DELETE_TYPES, &[&uuid.inner()]).await
    }

    async fn playlist_post(&self, playlist_post: PlaylistPost) -> Result<Playlist, Self::Error> {
        self.query_one(
            sql::UPSERT,
            sql::UPSERT_TYPES,
            convert::to_playlist,
            &[
                &Uuid::default().inner(),
                &playlist_post.title.clone(),
                &playlist_post.members.map(convert::to_inner).as_slice()
            ])
            .await
    }

    async fn playlist_put(
        &self,
        uuid: Uuid,
        playlist_post: PlaylistPost,
    ) -> Result<Playlist, Self::Error> {
        self.query_one(
            sql::UPSERT,
            sql::UPSERT_TYPES,
            convert::to_playlist,
            &[
                &uuid.inner(),
                &playlist_post.title.clone(),
                &playlist_post.members.map(convert::to_inner).as_slice()
            ])
            .await
    }
}

mod sql {
    use tokio_postgres::types::Type;

    pub const LIST: &str = "SELECT * FROM playlist ORDER BY title;";
    pub const LIST_TYPES: &[Type] = &[];

    pub const _LIST_FULL: &str = "SELECT playlist.id AS id, title, ARRAY_AGG(lyric_id ORDER BY ordering) members FROM playlist INNER JOIN member ON playlist.id = playlist_id GROUP BY playlist.id ORDER BY playlist.title;";

    pub const ITEM: &str = "SELECT playlist.id AS id, title, ARRAY_AGG(lyric_id ORDER BY ordering) members FROM playlist INNER JOIN member ON playlist.id = playlist_id GROUP BY playlist.id HAVING playlist.id = $1";
    pub const ITEM_TYPES: &[Type] = &[Type::UUID];

    pub const DELETE: &str = "DELETE FROM playlist WHERE id = $1;";
    pub const DELETE_TYPES: &[Type] = &[Type::UUID];

    pub const UPSERT: &str = "SELECT * from fn_upsert_playlist($1, $2, $3);";
    pub const UPSERT_TYPES: &[Type] = &[Type::UUID, Type::VARCHAR, Type::UUID_ARRAY];
}
