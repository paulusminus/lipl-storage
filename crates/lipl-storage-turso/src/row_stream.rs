use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{Stream, StreamExt, stream::unfold};
use pin_project::pin_project;
use turso::{Row, Rows};

type Result<T, E = turso::Error> = std::result::Result<T, E>;

#[allow(dead_code)]
pub trait IntoStream<S> {
    fn into_stream(self) -> S;
}

impl IntoStream<RowStream> for Rows {
    fn into_stream(self) -> RowStream {
        RowStream {
            rows: unfold(self, |mut rows: Rows| async move {
                let row = rows.next().await;
                convert(row).map(|s| (s, rows))
            })
            .boxed(),
        }
    }
}

#[pin_project]
pub struct RowStream {
    rows: Pin<Box<dyn Stream<Item = Result<Row>> + Send>>,
}

fn convert(row: Result<Option<Row>>) -> Option<Result<Row>> {
    match row {
        Ok(Some(row)) => Some(Ok(row)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    }
}

impl Stream for RowStream {
    type Item = Result<Row>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().rows.poll_next_unpin(cx)
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::to_lyric;

    use super::IntoStream;
    use futures_util::TryStreamExt;
    use lipl_core::Uuid;
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

        let mut rows = list_lyrics(&con).await.into_stream().map_ok(to_lyric);
        let lyric1 = rows.try_next().await.unwrap().unwrap().unwrap();
        let lyric2 = rows.try_next().await.unwrap().unwrap().unwrap();
        assert_eq!(lyric1.title, *"Er is er één jarig");
        assert_eq!(lyric2.title, *"Sinterklaas kapoentje");
    }
}
