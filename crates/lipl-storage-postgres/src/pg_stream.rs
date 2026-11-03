use std::{
    pin::Pin,
    task::{Context, Poll, ready},
};

use futures_util::{Stream, StreamExt};
use lipl_core::{Result, postgres_error};
use pin_project::pin_project;
use tokio_postgres::{Row, RowStream};

#[pin_project]
pub struct PgStream<O> {
    #[pin]
    s: RowStream,
    f: fn(Row) -> Result<O>,
}

pub fn objects<O>(f: fn(Row) -> Result<O>) -> impl Fn(RowStream) -> PgStream<O> {
    move |s| PgStream { s, f }
}

impl<O> Stream for PgStream<O> {
    type Item = Result<O>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        let row = ready!(this.s.poll_next_unpin(cx));
        Poll::Ready(row.map(|r| r.map_err(postgres_error).and_then(this.f)))
    }
}
