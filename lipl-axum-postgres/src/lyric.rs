use async_trait::async_trait;
use lipl_core::{Lyric, LyricDb, LyricPost, Summary, Uuid};
use parts::to_text;
use tokio_postgres::types::Type;

use super::convert;
use crate::error::Error;
use crate::PostgresConnection;

#[async_trait]
impl<'a> LyricDb for PostgresConnection {
    type Error = Error;

    async fn lyric_list(&self) -> Result<Vec<Summary>, Self::Error> {
        self.query(sql::LIST, &[], convert::to_summary, &[]).await
    }

    async fn lyric_item(&self, uuid: Uuid) -> Result<Lyric, Self::Error> {
        self.query_one(sql::ITEM, &[Type::UUID], convert::to_lyric, &[&uuid.inner()]).await
    }

    async fn lyric_post(&self, lyric_post: LyricPost) -> Result<Lyric, Self::Error> {
        self.query_one(
            sql::INSERT,
            &[Type::UUID, Type::VARCHAR, Type::VARCHAR],
            convert::to_lyric,
            &[&Uuid::default().inner(), &lyric_post.title.clone(), &to_text(&lyric_post.parts)],
        ).await
    }

    async fn lyric_delete(&self, uuid: Uuid) -> Result<(), Self::Error> {
        self.execute(sql::DELETE, &[Type::UUID], &[&uuid.inner()]).await
    }

    async fn lyric_put(&self, uuid: Uuid, lyric_post: LyricPost) -> Result<Lyric, Self::Error> {
        self.query_one(
            sql::UPDATE,
            &[Type::VARCHAR, Type::VARCHAR, Type::UUID],
            convert::to_lyric,
            &[&lyric_post.title.clone(), &to_text(&lyric_post.parts), &uuid.inner()]
        )
        .await
    }
}

mod sql {
    pub const LIST: &str = "SELECT * FROM lyric ORDER BY title;";
    pub const ITEM: &str = "SELECT * FROM lyric WHERE id = $1;";
    pub const DELETE: &str = "DELETE FROM lyric WHERE id = $1;";
    pub const INSERT: &str = "INSERT INTO lyric(id, title, parts) VALUES($1, $2, $3) RETURNING *;";
    pub const UPDATE: &str = "UPDATE lyric SET title = $1, parts = $2 WHERE id = $3 RETURNING *;";
}
