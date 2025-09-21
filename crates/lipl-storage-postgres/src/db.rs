use futures_util::future::BoxFuture;
use futures_util::{FutureExt, TryFutureExt};
use lipl_core::vec_ext::VecExt;
use lipl_core::{Error, LiplRepo, Lyric, Playlist, Result, Summary, Uuid, parts::to_text};

use super::convert;
use crate::PostgresConnectionPool;

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

impl LiplRepo for PostgresConnectionPool {
    fn get_lyric_summaries(&self) -> BoxFuture<'_, Result<Vec<Summary>>> {
        self.query(lyric::LIST, lyric::LIST_TYPES, convert::to_summary, &[])
            .err_into()
            .boxed()
    }

    fn get_lyrics(&self) -> BoxFuture<'_, Result<Vec<Lyric>>> {
        self.query(
            lyric::LIST_FULL,
            lyric::LIST_FULL_TYPES,
            convert::to_lyric,
            &[],
        )
        .err_into()
        .boxed()
    }

    fn get_lyric(&self, uuid: Uuid) -> BoxFuture<'_, Result<Lyric>> {
        async move {
            self.query_one(
                lyric::ITEM,
                lyric::ITEM_TYPES,
                convert::to_lyric,
                &[&uuid.inner()],
            )
            .map_err(pg_error_to_lipl_core(uuid))
            .await
        }
        .boxed()
    }

    fn upsert_lyric(&self, lyric: Lyric) -> BoxFuture<'_, Result<Lyric>> {
        async move {
            self.query_one(
                lyric::UPSERT,
                lyric::UPSERT_TYPES,
                convert::to_lyric,
                &[
                    &Uuid::default().inner(),
                    &lyric.title.clone(),
                    &to_text(&lyric.parts),
                ],
            )
            .err_into()
            .await
        }
        .boxed()
    }

    fn delete_lyric(&self, uuid: Uuid) -> BoxFuture<'_, Result<()>> {
        async move {
            let count = self
                .execute(lyric::DELETE, lyric::DELETE_TYPES, &[&uuid.inner()])
                .await?;
            error_on_count(count, uuid)
        }
        .boxed()
    }

    fn get_playlist_summaries(&self) -> BoxFuture<'_, Result<Vec<Summary>>> {
        self.query(
            playlist::LIST,
            playlist::LIST_TYPES,
            convert::to_summary,
            &[],
        )
        .err_into()
        .boxed()
    }

    fn get_playlists(&self) -> BoxFuture<'_, Result<Vec<Playlist>>> {
        self.query(
            playlist::LIST_FULL,
            playlist::LIST_FULL_TYPES,
            convert::to_playlist,
            &[],
        )
        .err_into()
        .boxed()
    }

    fn get_playlist(&self, uuid: Uuid) -> BoxFuture<'_, Result<Playlist>> {
        async move {
            self.query_one(
                playlist::ITEM,
                playlist::ITEM_TYPES,
                convert::to_playlist,
                &[&uuid.inner()],
            )
            .map_err(pg_error_to_lipl_core(uuid))
            .await
        }
        .boxed()
    }

    fn delete_playlist(&self, uuid: Uuid) -> BoxFuture<'_, Result<()>> {
        async move {
            let count = self
                .execute(playlist::DELETE, playlist::DELETE_TYPES, &[&uuid.inner()])
                .await?;
            error_on_count(count, uuid)
        }
        .boxed()
    }

    fn upsert_playlist(&self, playlist: Playlist) -> BoxFuture<'_, Result<Playlist>> {
        async move {
            self.query_one(
                playlist::UPSERT,
                playlist::UPSERT_TYPES,
                convert::to_playlist,
                &[
                    &Uuid::default().inner(),
                    &playlist.title.clone(),
                    &playlist.members.map(convert::to_inner).as_slice(),
                ],
            )
            .err_into()
            .await
        }
        .boxed()
    }

    fn stop(&self) -> BoxFuture<'_, Result<()>> {
        async move { Ok(()) }.boxed()
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
