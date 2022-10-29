use async_trait::async_trait;
use futures_util::TryFutureExt;
use lipl_types::{ext::VecExt, Playlist, PlaylistDb, PlaylistPost, Summary, Uuid};

use super::convert;
use crate::{error::Error, PostgresConnection};

#[async_trait]
impl<'a> PlaylistDb for PostgresConnection<'a> {
    type Error = Error;
    async fn playlist_list(&self) -> Result<Vec<Summary>, Self::Error> {
        self.inner
            .query(sql::LIST, &[])
            .map_err(Error::from)
            .map_ok(convert::to_list(convert::to_summary))
            .await
    }

    async fn playlist_item(&self, uuid: Uuid) -> Result<Playlist, Self::Error> {
        self.inner
            .query_one(sql::ITEM, &[&uuid.inner()])
            .map_err(Error::from)
            .map_ok(convert::to_playlist)
            .await
    }

    async fn playlist_delete(&self, uuid: Uuid) -> Result<(), Self::Error> {
        self.inner
            .execute(sql::DELETE, &[&uuid.inner()])
            .map_err(Error::from)
            .map_ok(|_| {})
            .await
    }

    async fn playlist_post(&self, playlist_post: PlaylistPost) -> Result<Playlist, Self::Error> {
        let id = Uuid::default();
        let members = playlist_post.members.map(convert::to_inner);

        self.inner
            .query_one(
                sql::UPSERT,
                &[&id.inner(), &playlist_post.title, &members.as_slice()],
            )
            .map_err(Error::from)
            // .inspect_ok(|row| { println!("Row: {:#?}", row.get::<&str, Option<Vec<uuid::Uuid>>>("fn_upsert_playlist")); })
            .await?;

        self.playlist_item(id).await
    }

    async fn playlist_put(
        &self,
        uuid: Uuid,
        playlist_post: PlaylistPost,
    ) -> Result<Playlist, Self::Error> {
        let members = playlist_post.members.map(convert::to_inner);

        self.inner
            .query_one(
                sql::UPSERT,
                &[&uuid.inner(), &playlist_post.title, &members.as_slice()],
            )
            .map_err(Error::from)
            .await?;

        self.playlist_item(uuid).await
    }
}

mod sql {
    pub const LIST: &str = "SELECT * FROM playlist ORDER BY title;";
    pub const _LIST_FULL: &str = "SELECT playlist.id AS id, title, ARRAY_AGG(lyric_id ORDER BY ordering) members FROM playlist INNER JOIN member ON playlist.id = playlist_id GROUP BY playlist.id ORDER BY playlist.title;";
    pub const ITEM: &str = "SELECT playlist.id AS id, title, ARRAY_AGG(lyric_id ORDER BY ordering) members FROM playlist INNER JOIN member ON playlist.id = playlist_id GROUP BY playlist.id HAVING playlist.id = $1";
    pub const DELETE: &str = "DELETE FROM playlist WHERE id = $1;";
    pub const UPSERT: &str = "SELECT fn_upsert_playlist($1, $2, $3);";
}
