use async_trait::async_trait;
use lipl_core::{Lyric, LyricDb, LyricPost, Summary, Uuid};
use parts::to_text;

use super::convert;
use crate::error::Error;
use crate::PostgresConnectionPool;

#[async_trait]
impl LyricDb for PostgresConnectionPool {
    type Error = Error;

    async fn lyric_list(&self) -> Result<Vec<Summary>, Self::Error> {
        self.query(sql::LIST, sql::LIST_TYPES, convert::to_summary, &[]).await
    }

    async fn lyric_list_full(&self) -> Result<Vec<Lyric>, Self::Error> {
        self.query(sql::LIST_FULL, sql::LIST_FULL_TYPES, convert::to_lyric, &[]).await
    }

    async fn lyric_item(&self, uuid: Uuid) -> Result<Lyric, Self::Error> {
        self.query_one(sql::ITEM, sql::ITEM_TYPES, convert::to_lyric, &[&uuid.inner()]).await
    }

    async fn lyric_post(&self, lyric_post: LyricPost) -> Result<Lyric, Self::Error> {
        self.query_one(
            sql::INSERT,
            sql::INSERT_TYPES,
            convert::to_lyric,
            &[&Uuid::default().inner(), &lyric_post.title.clone(), &to_text(&lyric_post.parts)],
        ).await
    }

    async fn lyric_delete(&self, uuid: Uuid) -> Result<(), Self::Error> {
        self.execute(sql::DELETE, sql::DELETE_TYPES, &[&uuid.inner()]).await
    }

    async fn lyric_put(&self, uuid: Uuid, lyric_post: LyricPost) -> Result<Lyric, Self::Error> {
        self.query_one(
            sql::UPDATE,
            sql::UPDATE_TYPES,
            convert::to_lyric,
            &[&lyric_post.title.clone(), &to_text(&lyric_post.parts), &uuid.inner()]
        )
        .await
    }
}

mod sql {
    use tokio_postgres::types::Type;

    pub const LIST: &str = "SELECT id, title FROM lyric ORDER BY title;";
    pub const LIST_TYPES: &[Type] = &[];

    pub const LIST_FULL: &str = "SELECT id, title, parts FROM lyric ORDER BY title;";
    pub const LIST_FULL_TYPES: &[Type] = &[];

    pub const ITEM: &str = "SELECT * FROM lyric WHERE id = $1;";
    pub const ITEM_TYPES: &[Type] = &[Type::UUID];

    pub const DELETE: &str = "DELETE FROM lyric WHERE id = $1;";
    pub const DELETE_TYPES: &[Type] = &[Type::UUID];

    pub const INSERT: &str = "INSERT INTO lyric(id, title, parts) VALUES($1, $2, $3) RETURNING *;";
    pub const INSERT_TYPES: &[Type] = &[Type::UUID, Type::VARCHAR, Type::VARCHAR];

    pub const UPDATE: &str = "UPDATE lyric SET title = $1, parts = $2 WHERE id = $3 RETURNING *;";
    pub const UPDATE_TYPES: &[Type] = &[Type::VARCHAR, Type::VARCHAR, Type::UUID];
}
