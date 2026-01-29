use lipl_core::{Lyric, Playlist, Result, Summary, Uuid, postgres_error};
use tokio_stream::wrappers::ReceiverStream;
use turso::{Row, Rows};

pub fn to_list<T: Send + Sync + 'static>(
    f: fn(Row) -> Result<T>,
) -> impl Fn(Rows) -> Result<ReceiverStream<Result<T>>>
where
{
    move |mut rows| {
        let (tx, rx) = tokio::sync::mpsc::channel::<Result<T>>(20);
        tokio::task::spawn(async move {
            while let Some(row) = rows.next().await.map_err(postgres_error)? {
                tx.send(f(row)).await.map_err(postgres_error)?;
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
            .map_err(postgres_error)
            .and_then(|s| s.parse::<Uuid>())?
            .into(),
        title: row.get::<String>(1).map_err(postgres_error)?,
        parts: lipl_core::parts::to_parts(&row.get::<String>(2).map_err(postgres_error)?),
    })
}

pub fn to_playlist(row: Row) -> Result<Playlist> {
    let id = row
        .get::<String>(0)
        .map_err(postgres_error)
        .and_then(|s| s.parse::<Uuid>())?;
    let title = row.get::<String>(1).map_err(postgres_error)?;
    let members = row
        .get::<String>(2)
        .map_err(postgres_error)
        .and_then(|members| {
            members
                .split(',')
                .map(|member| member.parse::<Uuid>())
                .collect::<Result<Vec<Uuid>>>()
        })?;
    Ok(Playlist { id, title, members })
}

pub fn to_summary(row: Row) -> Result<Summary> {
    Ok(Summary {
        id: row
            .get::<String>(0)
            .map_err(postgres_error)
            .and_then(|s| s.parse::<Uuid>())?
            .into(),
        title: row.get::<String>(1).map_err(postgres_error)?,
    })
}
