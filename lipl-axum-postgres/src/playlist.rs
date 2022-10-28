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
        let title = self
            .inner
            .query_one(sql::ITEM_TITLE, &[&uuid.inner()])
            .map_err(Error::from)
            .map_ok(convert::to_title)
            .await?;

        let members = self
            .inner
            .query(sql::ITEM_MEMBERS, &[&uuid.inner()])
            .map_err(Error::from)
            .map_ok(convert::to_list(convert::to_uuid))
            .await?;

        Ok(Playlist {
            id: uuid,
            title,
            members,
        })
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
    pub const ITEM_TITLE: &str = "SELECT title FROM playlist WHERE id = $1;";
    pub const ITEM_MEMBERS: &str =
        "SELECT lyric_id FROM membership WHERE playlist_id = $1 ORDER BY ordering;";
    pub const DELETE: &str = "DELETE FROM playlist WHERE id = $1;";
    pub const UPSERT: &str = "SELECT fn_upsert_playlist($1, $2, $3);";
}
