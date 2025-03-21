use crate::Uuid;
use futures_channel::oneshot::Canceled;
use futures_channel::{mpsc, oneshot};
use lipl_core::Error;
use lipl_core::transaction::Request;
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
    f: fn(oneshot::Sender<Result<T>>) -> Request,
) -> Result<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<T>>();
    tx.try_send(f(oneshot_tx)).map_err(send_failed)?;
    oneshot_rx.await.map_err(canceled)?
}

pub async fn select_by_id<T>(
    mut tx: mpsc::Sender<Request>,
    uuid: Uuid,
    f: fn(Uuid, oneshot::Sender<Result<T>>) -> Request,
) -> Result<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<T>>();
    tx.try_send(f(uuid, oneshot_tx)).map_err(send_failed)?;
    oneshot_rx.await.map_err(canceled)?
}

pub async fn delete_by_id(
    mut tx: mpsc::Sender<Request>,
    uuid: Uuid,
    f: fn(Uuid, oneshot::Sender<Result<()>>) -> Request,
) -> Result<()> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<()>>();
    tx.try_send(f(uuid, oneshot_tx)).map_err(send_failed)?;
    oneshot_rx.await.map_err(canceled)?
}

pub async fn post<T: Debug>(
    mut tx: mpsc::Sender<Request>,
    t: T,
    f: fn(T, oneshot::Sender<Result<T>>) -> Request,
) -> Result<T> {
    let (oneshot_tx, oneshot_rx) = oneshot::channel::<Result<T>>();
    tx.try_send(f(t, oneshot_tx)).map_err(send_failed)?;
    oneshot_rx.await.map_err(canceled)?
}
