use crate::{Uuid, Summary, LyricPost, PlaylistPost, Lyric, Playlist, RepoResult, RepoError};
use futures::channel::{mpsc, oneshot};

pub enum Request {
    LyricSummaries(oneshot::Sender<RepoResult<Vec<Summary>>>),
    LyricList(oneshot::Sender<RepoResult<Vec<Lyric>>>),
    LyricItem(Uuid, oneshot::Sender<RepoResult<Lyric>>),
    LyricDelete(Uuid, oneshot::Sender<RepoResult<()>>),
    LyricPost(LyricPost, oneshot::Sender<RepoResult<()>>),
    LyricPut(Uuid, LyricPost, oneshot::Sender<RepoResult<()>>),
    PlaylistSummaries(oneshot::Sender<RepoResult<Vec<Summary>>>),
    PlaylistList(oneshot::Sender<RepoResult<Vec<Playlist>>>),
    PlaylistItem(Uuid, oneshot::Sender<RepoResult<Playlist>>),
    PlaylistDelete(Uuid, oneshot::Sender<RepoResult<()>>),
    PlaylistPost(PlaylistPost, oneshot::Sender<RepoResult<()>>),
    PlaylistPut(Uuid, PlaylistPost, oneshot::Sender<RepoResult<()>>),
}

pub async fn send<T>(tx: &mut mpsc::Sender<Request>, f: impl Fn(oneshot::Sender<Result<T, RepoError>>) -> Request) -> RepoResult<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<RepoResult<T>>();
    tx.try_send(f(oneshot_tx))?;
    oneshot_rx.await?
}
