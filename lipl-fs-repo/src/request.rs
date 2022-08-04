use crate::{Uuid, Summary, Lyric, Playlist, RepoResult};
use futures::channel::{mpsc, oneshot};
use lipl_types::RepoError;

type ResultSender<T> = oneshot::Sender<RepoResult<T>>;

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

pub async fn select<T>(tx: &mut mpsc::Sender<Request>, f: impl Fn(ResultSender<T>) -> Request) -> RepoResult<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<RepoResult<T>>();
    tx.try_send(f(oneshot_tx)).map_err(|_| RepoError::SendFailed("".to_owned()))?;
    oneshot_rx.await?
}

pub async fn select_by_id<T>(tx: &mut mpsc::Sender<Request>, uuid: Uuid, f: impl Fn(Uuid, ResultSender<T>) -> Request) -> RepoResult<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<RepoResult<T>>();
    tx.try_send(f(uuid, oneshot_tx)).map_err(|_| RepoError::SendFailed("".to_owned()))?;
    oneshot_rx.await?
}

pub async fn delete_by_id(tx: &mut mpsc::Sender<Request>, uuid: Uuid, f: impl Fn(Uuid, ResultSender<()>) -> Request) -> RepoResult<()> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<RepoResult<()>>();
    tx.try_send(f(uuid, oneshot_tx)).map_err(|_| RepoError::SendFailed("".to_owned()))?;
    oneshot_rx.await?
}

pub async fn post<T>(tx: &mut mpsc::Sender<Request>, t: T, f: impl Fn(T, ResultSender<T>) -> Request) -> RepoResult<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<RepoResult<T>>();
    tx.try_send(f(t, oneshot_tx)).map_err(|_| RepoError::SendFailed("".to_owned()))?;
    oneshot_rx.await?
}
