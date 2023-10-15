use crate::Uuid;
use futures::channel::oneshot::Canceled;
use futures::channel::{mpsc, oneshot};
use lipl_core::transaction::{Request, ResultSender};
use lipl_core::Error;
use std::fmt::Debug;

type Result<T> = std::result::Result<T, Error>;

fn send_failed<E: std::error::Error>(error: E) -> Error {
    Error::SendFailed(error.to_string())
}

fn canceled(error: Canceled) -> Error {
    Error::Canceled(error.into())
}

pub async fn select<T>(
    mut tx: mpsc::Sender<Request>,
    f: fn(ResultSender<T>) -> Request,
) -> Result<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<T>>();
    tx.try_send(f(oneshot_tx)).map_err(send_failed)?;
    oneshot_rx.await.map_err(canceled)?
}

pub async fn select_by_id<T>(
    mut tx: mpsc::Sender<Request>,
    uuid: Uuid,
    f: fn(Uuid, ResultSender<T>) -> Request,
) -> Result<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<T>>();
    tx.try_send(f(uuid, oneshot_tx)).map_err(send_failed)?;
    oneshot_rx.await.map_err(canceled)?
}

pub async fn delete_by_id(
    mut tx: mpsc::Sender<Request>,
    uuid: Uuid,
    f: fn(Uuid, ResultSender<()>) -> Request,
) -> Result<()> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<()>>();
    tx.try_send(f(uuid, oneshot_tx)).map_err(send_failed)?;
    oneshot_rx.await.map_err(canceled)?
}

pub async fn post<T: Debug>(
    mut tx: mpsc::Sender<Request>,
    t: T,
    f: fn(T, ResultSender<T>) -> Request,
) -> Result<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<T>>();
    tx.try_send(f(t, oneshot_tx)).map_err(send_failed)?;
    oneshot_rx.await.map_err(canceled)?
}
