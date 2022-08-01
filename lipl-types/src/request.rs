use crate::{Uuid, Summary, LyricPost, PlaylistPost, Lyric, Playlist, RepoResult};
use futures::channel::{mpsc, oneshot};

type ResultSender<T> = oneshot::Sender<RepoResult<T>>;

pub enum Request {
    LyricSummaries(ResultSender<Vec<Summary>>),
    LyricList(ResultSender<Vec<Lyric>>),
    LyricItem(Uuid, ResultSender<Lyric>),
    LyricDelete(Uuid, ResultSender<()>),
    LyricPost(LyricPost, ResultSender<()>),
    LyricPut(Uuid, LyricPost, ResultSender<()>),
    PlaylistSummaries(ResultSender<Vec<Summary>>),
    PlaylistList(ResultSender<Vec<Playlist>>),
    PlaylistItem(Uuid, ResultSender<Playlist>),
    PlaylistDelete(Uuid, ResultSender<()>),
    PlaylistPost(PlaylistPost, ResultSender<()>),
    PlaylistPut(Uuid, PlaylistPost, ResultSender<()>),
}

pub async fn send<T>(tx: &mut mpsc::Sender<Request>, f: impl Fn(ResultSender<T>) -> Request) -> RepoResult<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<RepoResult<T>>();
    tx.try_send(f(oneshot_tx))?;
    oneshot_rx.await?
}
