use async_trait::async_trait;
use futures_util::TryFutureExt;
use lipl_types::{Lyric, LyricDb, LyricPost, Summary, Uuid};
use parts::to_text;

use super::convert;
use crate::error::Error;
use crate::{Result, PostgresConnection};

#[async_trait]
impl<'a> LyricDb for PostgresConnection<'a> {
    type Error = Error;
    async fn lyric_list(&self) -> Result<Vec<Summary>> {
        self.inner.query(sql::LIST, &[])
            .map_err(Error::from)
            .map_ok(convert::to_list(convert::to_summary))
            .await
    }

    async fn lyric_item(&self, uuid: Uuid) -> Result<Lyric> {
        self.inner.query_one(sql::ITEM, &[&uuid.inner()])
            .map_err(Error::from)
            .map_ok(convert::to_lyric)
            .await
    }

    async fn lyric_post(&self, lyric_post: LyricPost) -> Result<Lyric> {
        let id = Uuid::default();
        self.inner.execute(
            sql::INSERT,
            &[&id.inner(), &lyric_post.title, &to_text(&lyric_post.parts)],
        )
        .map_err(Error::from)
        .await?;

        self.lyric_item(id).await
    }

    async fn lyric_delete(&self, uuid: Uuid) -> Result<()> {
        self.inner.execute(sql::DELETE, &[&uuid.inner()])
            .map_err(Error::from)
            .map_ok(convert::to_unit)
            .await
    }

    async fn lyric_put(&self, uuid: Uuid, lyric_post: LyricPost) -> Result<Lyric> {
        self.inner.execute(
            sql::UPDATE,
            &[
                &lyric_post.title,
                &to_text(&lyric_post.parts),
                &uuid.inner(),
            ],
        )
        .map_err(Error::from)
        .await?;

        self.lyric_item(uuid).await
    }
}

mod sql {
    pub const LIST: &str = "SELECT * FROM lyric ORDER BY title;";
    pub const ITEM: &str = "SELECT * FROM lyric WHERE id = $1;";
    pub const DELETE: &str = "DELETE FROM lyric WHERE id = $1;";
    pub const INSERT: &str = "INSERT INTO lyric(id, title, parts) VALUES($1, $2, $3);";
    pub const UPDATE: &str = "UPDATE lyric SET title = $1, parts = $2 WHERE id = $3;";
}
