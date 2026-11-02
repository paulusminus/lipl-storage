use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{Stream, StreamExt};
use lipl_core::Result;
use turso::Row;

use crate::{ErrInto, row_stream::RowStream};

pub struct ObjectStream<O> {
    s: RowStream,
    f: fn(Row) -> lipl_core::Result<O>,
}

impl<O> ObjectStream<O> {
    pub fn new(s: RowStream, f: fn(Row) -> Result<O>) -> Self {
        Self { s, f }
    }
}

impl<O> Stream for ObjectStream<O> {
    type Item = Result<O>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.s.poll_next_unpin(cx) {
            Poll::Ready(row) => Poll::Ready(row.map(|r| r.err_into().and_then(self.f))),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(test)]
mod test {
    use futures_util::TryStreamExt;
    use lipl_core::{Lyric, Uuid};
    use turso::params;

    use crate::{convert::to_lyric, object_stream::ObjectStream, row_stream::RowStream};

    async fn create_db() -> turso::Connection {
        let db = turso::Builder::new_local(":memory:").build().await.unwrap();
        let con = db.connect().unwrap();
        con.execute_batch(include_str!("create_db.sql"))
            .await
            .unwrap();
        con
    }

    async fn insert_lyric(con: &turso::Connection, title: &str, parts: &str) {
        con.execute(
            "INSERT INTO lyric (id, title, parts) VALUES ($1, $2, $3)",
            params!(Uuid::default().to_string().as_str(), title, parts),
        )
        .await
        .unwrap();
    }

    async fn list_lyrics(con: &turso::Connection) -> ObjectStream<Lyric> {
        con.query(
            "SELECT id, title, parts FROM lyric ORDER BY title",
            params!(),
        )
        .await
        .map(RowStream::from)
        .map(|s| ObjectStream::new(s, to_lyric))
        .unwrap()
    }

    #[tokio::test]
    async fn object_stream() {
        let con = create_db().await;
        insert_lyric(&con, "Sinterklaas kapoentje", "").await;
        insert_lyric(&con, "Er is er één jarig", "").await;

        let mut object_stream = list_lyrics(&con).await;
        let lyric1 = object_stream.try_next().await.unwrap().unwrap();
        let lyric2 = object_stream.try_next().await.unwrap().unwrap();
        assert_eq!(lyric1.title, *"Er is er één jarig");
        assert_eq!(lyric2.title, *"Sinterklaas kapoentje");
    }
}
