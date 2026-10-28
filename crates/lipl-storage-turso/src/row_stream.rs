use std::{pin::pin, task::Poll};

use futures_util::{Stream, TryFutureExt};
use pin_project::pin_project;
use turso::{Row, Rows};

type Result<T, E = turso::Error> = std::result::Result<T, E>;

#[pin_project]
struct RowStream {
    #[pin]
    rows: Rows,
}

impl From<Rows> for RowStream {
    fn from(rows: Rows) -> Self {
        Self { rows }
    }
}

impl Stream for RowStream {
    type Item = Result<Row>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.project();
        let f = this.rows.next();
        let mut pinned = pin!(f);
        match pinned.try_poll_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(row) => match row {
                Ok(None) => Poll::Ready(None),
                Ok(Some(row)) => Poll::Ready(Some(Ok(row))),
                Err(error) => Poll::Ready(Some(Err(error))),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RowStream;
    use crate::convert::to_lyric;
    use futures_util::TryStreamExt;
    use lipl_core::Uuid;
    use std::pin::pin;
    use turso::{Rows, params};

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

    async fn list_lyrics(con: &turso::Connection) -> Rows {
        con.query(
            "SELECT id, title, parts FROM lyric ORDER BY title",
            params!(),
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_row_stream() {
        let con = create_db().await;
        insert_lyric(&con, "Sinterklaas kapoentje", "").await;
        insert_lyric(&con, "Er is er één jarig", "").await;

        let row_stream: RowStream = list_lyrics(&con).await.into();
        let mut rows = row_stream.map_ok(to_lyric);
        let lyric1 = rows.try_next().await.unwrap().unwrap().unwrap();
        let lyric2 = rows.try_next().await.unwrap().unwrap().unwrap();
        assert_eq!(lyric1.title, *"Er is er één jarig");
        assert_eq!(lyric2.title, *"Sinterklaas kapoentje");
    }

    #[tokio::test]
    async fn test_rowstream2() {
        let con = create_db().await;
        insert_lyric(&con, "Sinterklaas kapoentje", "").await;
        insert_lyric(&con, "Er is er één jarig", "").await;

        let rows = list_lyrics(&con).await;
        let row_stream: RowStream = rows.into();
        let mut pinned = pin!(row_stream);
        while let Some(row) = pinned.try_next().await.unwrap() {
            dbg!(row);
        }
    }
}
