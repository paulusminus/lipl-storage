use std::fmt::Debug;

use crate::{Uuid, Summary, Lyric, Playlist};
use futures::channel::{mpsc, oneshot};
use crate::FileRepoError;

type Result<T> = std::result::Result<T, FileRepoError>;
type ResultSender<T> = oneshot::Sender<Result<T>>;

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

pub async fn select<T>(mut tx: mpsc::Sender<Request>, f: fn(ResultSender<T>) -> Request) -> Result<T>
{
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<T>>();
    tx.try_send(f(oneshot_tx)).map_err(|_| FileRepoError::SendFailed)?;
    oneshot_rx.await?
}

pub async fn select_by_id<T>(mut tx: mpsc::Sender<Request>, uuid: Uuid, f: fn(Uuid, ResultSender<T>) -> Request) -> Result<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<T>>();
    tx.try_send(f(uuid, oneshot_tx)).map_err(|_| FileRepoError::SendFailed)?;
    oneshot_rx.await?
}

pub async fn delete_by_id(mut tx: mpsc::Sender<Request>, uuid: Uuid, f: fn(Uuid, ResultSender<()>) -> Request) -> Result<()> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<()>>();
    tx.try_send(f(uuid, oneshot_tx)).map_err(|_| FileRepoError::SendFailed)?;
    oneshot_rx.await?
}

pub async fn post<T: Debug>(mut tx: mpsc::Sender<Request>, t: T, f: fn(T, ResultSender<T>) -> Request) -> Result<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<T>>();
    tx.try_send(f(t, oneshot_tx)).map_err(|_| FileRepoError::SendFailed)?;
    oneshot_rx.await?
}
