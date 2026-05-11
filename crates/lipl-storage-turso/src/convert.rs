use lipl_core::{Lyric, Playlist, Result, Summary, Uuid, parts::to_parts};
use tokio_stream::wrappers::ReceiverStream;
use turso::{Row, Rows};

use crate::ErrInto;

fn to_uuid(s: impl AsRef<str>) -> Result<Uuid> {
    s.as_ref().parse::<Uuid>().err_into()
}

trait RowExt {
    fn get_uuid(&self, index: usize) -> Result<Uuid>;
    fn get_uuids(&self, index: usize) -> Result<Vec<Uuid>>;
    fn get_string(&self, index: usize) -> Result<String>;
    fn get_parts(&self, index: usize) -> Result<Vec<Vec<String>>>;
}

impl RowExt for Row {
    fn get_uuid(&self, index: usize) -> Result<Uuid> {
        self.get_string(index).and_then(to_uuid)
    }

    fn get_uuids(&self, index: usize) -> Result<Vec<Uuid>> {
        self.get_string(index)
            .and_then(|s| s.split(',').map(to_uuid).collect())
    }

    fn get_string(&self, index: usize) -> Result<String> {
        self.get::<String>(index).err_into()
    }

    fn get_parts(&self, index: usize) -> Result<Vec<Vec<String>>> {
        self.get_string(index).map(to_parts)
    }
}

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
        Ok(rx.into())
    }
}

pub fn to_lyric(row: Row) -> Result<Lyric> {
    row.get_uuid(0)
        .and_then(|id| row.get_string(1).map(|title| (id, title)))
        .and_then(|(id, title)| row.get_parts(2).map(|parts| Lyric { id, title, parts }))
}

pub fn to_playlist(row: Row) -> Result<Playlist> {
    Ok(Playlist {
        id: row.get_uuid(0)?,
        title: row.get_string(1)?,
        members: row.get_uuids(2)?,
    })
}

pub fn to_summary(row: Row) -> Result<Summary> {
    Ok(Summary {
        id: row.get_uuid(0)?,
        title: row.get_string(1)?,
    })
}
