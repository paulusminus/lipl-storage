use async_trait::async_trait;
use futures_util::TryFutureExt;
use lipl_types::{ext::VecExt, Playlist, PlaylistDb, PlaylistPost, Summary, Uuid};
use tokio_postgres::types::Type;

use super::convert;
use crate::{error::Error, PostgresConnection};

#[async_trait]
impl<'a> PlaylistDb for PostgresConnection<'a> {
    type Error = Error;
    async fn playlist_list(&self) -> Result<Vec<Summary>, Self::Error> {
        self.inner
            .prepare_typed(sql::LIST, &[])
            .and_then(|statement| async move {
                self.inner.query(&statement, &[]).await
            })
            // .query(sql::LIST, &[])
            .map_err(Error::from)
            .await
            .and_then(convert::to_list(convert::to_summary))
    }

    async fn playlist_item(&self, uuid: Uuid) -> Result<Playlist, Self::Error> {
        self.inner
            .prepare_typed(sql::ITEM, &[Type::UUID])
            .and_then(|statement| async move {
                self.inner.query_one(&statement, &[&uuid.inner()]).await
            })
            // .query_one(sql::ITEM, &[&uuid.inner()])
            .map_err(Error::from)
            .await
            .and_then(convert::to_playlist)
    }

    async fn playlist_delete(&self, uuid: Uuid) -> Result<(), Self::Error> {
        self.inner
            .prepare_typed(sql::DELETE, &[Type::UUID])
            .and_then(|statement| async move {
                self.inner.execute(&statement, &[&uuid.inner()]).await
            })
            // .execute(sql::DELETE, &[&uuid.inner()])
            .map_err(Error::from)
            .map_ok(|_| {})
            .await
    }

    async fn playlist_post(&self, playlist_post: PlaylistPost) -> Result<Playlist, Self::Error> {
        self.inner
            .prepare_typed(sql::UPSERT, &[Type::UUID, Type::VARCHAR, Type::UUID_ARRAY])
            .and_then(|statement| async move {
                self.inner.query_one(
                    &statement,
                    &[
                        &Uuid::default().inner(),
                        &playlist_post.title.clone(),
                        &playlist_post.members.map(convert::to_inner).as_slice()
                    ],    
                )
                .await
            })
            // .query_one(
            //     sql::UPSERT,
            //     &[
            //         &Uuid::default().inner(),
            //         &playlist_post.title.clone(),
            //         &playlist_post.members.map(convert::to_inner).as_slice()
            //     ],
            // )
            .map_err(Error::from)
            .await
            .and_then(convert::to_playlist)
    }

    async fn playlist_put(
        &self,
        uuid: Uuid,
        playlist_post: PlaylistPost,
    ) -> Result<Playlist, Self::Error> {
        self.inner
            .prepare_typed(sql::UPSERT, &[Type::UUID, Type::VARCHAR, Type::UUID_ARRAY])
            .and_then(|statement| async move {
                self.inner.query_one(
                    &statement,
                    &[
                        &uuid.inner(),
                        &playlist_post.title.clone(),
                        &playlist_post.members.map(convert::to_inner).as_slice()
                    ],
                )
                .await
            })
            // .query_one(
            //     sql::UPSERT,
            //     &[
            //         &uuid.inner(),
            //         &playlist_post.title.clone(),
            //         &playlist_post.members.map(convert::to_inner).as_slice()
            //     ],
            // )
            .map_err(Error::from)
            .await
            .and_then(convert::to_playlist)
    }
}

mod sql {
    pub const LIST: &str = "SELECT * FROM playlist ORDER BY title;";
    pub const _LIST_FULL: &str = "SELECT playlist.id AS id, title, ARRAY_AGG(lyric_id ORDER BY ordering) members FROM playlist INNER JOIN member ON playlist.id = playlist_id GROUP BY playlist.id ORDER BY playlist.title;";
    pub const ITEM: &str = "SELECT playlist.id AS id, title, ARRAY_AGG(lyric_id ORDER BY ordering) members FROM playlist INNER JOIN member ON playlist.id = playlist_id GROUP BY playlist.id HAVING playlist.id = $1";
    pub const DELETE: &str = "DELETE FROM playlist WHERE id = $1;";
    pub const UPSERT: &str = "SELECT * from fn_upsert_playlist($1, $2, $3);";
}
