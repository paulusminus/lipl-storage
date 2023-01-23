use chrono::SecondsFormat;
use serde::{Deserialize, Serialize};
use crate::{Lyric, Playlist, Summary, Uuid};

pub type ResultSender<T> = futures::channel::oneshot::Sender<crate::Result<T>>;

#[derive(Debug)]
pub enum Request {
    LyricSummaries(ResultSender<Vec<Summary>>),
    LyricList(ResultSender<Vec<Lyric>>),
    LyricItem(Uuid, ResultSender<Lyric>),
    LyricDelete(Uuid, ResultSender<()>),
    LyricPost(Lyric, ResultSender<Lyric>),
    PlaylistSummaries(ResultSender<Vec<Summary>>),
    PlaylistList(ResultSender<Vec<Playlist>>),
    PlaylistItem(Uuid, ResultSender<Playlist>),
    PlaylistDelete(Uuid, ResultSender<()>),
    PlaylistPost(Playlist, ResultSender<Playlist>),
    Stop(ResultSender<()>),
}

#[derive(Deserialize, Serialize)]
pub enum Transaction {
    LyricDelete(Uuid),
    LyricUpsert(Lyric),
    PlaylistDelete(Uuid),
    PlaylistUpsert(Playlist),
}

fn json_error<E: std::error::Error + Send + Sync + 'static>(error: E) -> crate::Error {
    crate::Error::Json(
        Box::new(error)
    )
}

pub fn log_to_traction<W>(mut f: W) -> impl FnMut(&Request) 
where
    W: std::io::Write,
{
    move |request| {
        let mut write = |transaction: Transaction| {
            serde_json::to_string(&(chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true), transaction))
                .map_err(json_error)
                .and_then(|json| f.write_fmt(format_args!("{}\n", json)).map_err(crate::Error::from))
                .and_then(|_| f.flush().map_err(crate::Error::from))
        };
        let result = match request {
            Request::LyricDelete(uuid, _) => {
                write(Transaction::LyricDelete(*uuid))
            },
            Request::LyricPost(lyric, _) => {
                write(Transaction::LyricUpsert(lyric.clone()))
            },
            Request::PlaylistDelete(uuid, _) => {
                write(Transaction::PlaylistDelete(*uuid))
            },
            Request::PlaylistPost(playlist, _) => {
                write(Transaction::PlaylistUpsert(playlist.clone()))
            },
            _ => Ok(()),
        };
        if let Err(error) = result {
            tracing::error!("Could not write to transaction log: {error}");
        }
    }
}
