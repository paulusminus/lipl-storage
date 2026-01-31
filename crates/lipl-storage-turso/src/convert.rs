use lipl_core::{Lyric, Playlist, Result, Summary, Uuid};
use tokio_stream::wrappers::ReceiverStream;
use turso::{Row, Rows};

use crate::ErrInto;

pub fn to_list<T: Send + Sync + 'static>(
    f: fn(Row) -> Result<T>,
) -> impl Fn(Rows) -> Result<ReceiverStream<Result<T>>>
where
{
    move |mut rows| {
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<T>>(20);
        tokio::task::spawn(async move {
            while let Some(row) = rows.next().await.err_into()? {
                tx.send(f(row)).await.err_into()?;
            }
            Ok::<_, lipl_core::Error>(())
        });
        Ok(ReceiverStream::new(rx))
    }
}

pub fn to_lyric(row: Row) -> Result<Lyric> {
    Ok(Lyric {
        id: row
            .get::<String>(0)
            .err_into()
            .and_then(|s| s.parse::<Uuid>())?,
        title: row.get::<String>(1).err_into()?,
        parts: lipl_core::parts::to_parts(&row.get::<String>(2).err_into()?),
    })
}

pub fn to_playlist(row: Row) -> Result<Playlist> {
    let id = row
        .get::<String>(0)
        .err_into()
        .and_then(|s| s.parse::<Uuid>())?;
    let title = row.get::<String>(1).err_into()?;
    let nullable_row = row.get_value(2).err_into()?;
    let members = if nullable_row.is_null() {
        vec![]
    } else {
        nullable_row
            .as_text()
            .ok_or(turso::Error::QueryReturnedNoRows)
            .err_into()?
            .split(',')
            .map(|member| member.parse::<Uuid>())
            .collect::<Result<Vec<Uuid>>>()?
    };
    Ok(Playlist { id, title, members })
}

pub fn to_summary(row: Row) -> Result<Summary> {
    row.get::<String>(0)
        .err_into()
        .and_then(|s| s.parse::<Uuid>())
        .and_then(|id| {
            row.get::<String>(1)
                .err_into()
                .map(|title| Summary { id, title })
        })
}
