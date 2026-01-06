use crate::{Error, Lyric, Playlist, Repo, Summary, Uuid};
use chrono::SecondsFormat;
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    pin::Pin,
    thread::JoinHandle,
};

pub type ResultSender<T> = futures_channel::oneshot::Sender<crate::Result<T>>;
pub type OptionalTransaction = Option<Transaction>;
type LogRecord = (String, Transaction);

pub enum RequestNew {
    LyricSummaries,
    LyricList,
    LyricItem(Uuid),
    LyricDelete(Uuid),
    LyricPost(Lyric),
    PlaylistSummaries,
    PlaylistList,
    PlaylistItem(Uuid),
    PlaylistDelete(Uuid),
    PlaylistPost(Playlist),
}

#[derive(Debug)]
pub enum Request {
    LyricSummaries(ResultSender<Vec<Summary>>),
    LyricList(ResultSender<Vec<Lyric>>),
    LyricListStream(ResultSender<Pin<Box<dyn Stream<Item = Result<Lyric, Error>>>>>),
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

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&(now(), self)).unwrap())
    }
}

impl std::str::FromStr for Transaction {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<LogRecord>(s)
            .map(|(_, transaction)| transaction)
            .map_err(Box::new)
            .map_err(to_json_error)
    }
}

impl From<&Request> for OptionalTransaction {
    fn from(request: &Request) -> Self {
        match request {
            Request::LyricDelete(uuid, _) => Some(Transaction::LyricDelete(*uuid)),
            Request::LyricPost(lyric, _) => Some(Transaction::LyricUpsert(lyric.clone())),
            Request::PlaylistDelete(uuid, _) => Some(Transaction::PlaylistDelete(*uuid)),
            Request::PlaylistPost(playlist, _) => {
                Some(Transaction::PlaylistUpsert(playlist.clone()))
            }
            _ => None,
        }
    }
}

fn to_json_error<E>(error: E) -> crate::Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    crate::Error::Json(Box::new(error))
}

fn now() -> String {
    chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true)
}

fn write<W>(w: &mut W, json: String) -> crate::Result<()>
where
    W: std::io::Write,
{
    w.write_fmt(format_args!("{json}\n"))?;
    w.flush()?;
    Ok(())
}

fn line_to_transaction(line: std::io::Result<String>) -> crate::Result<Transaction> {
    line.map_err(crate::Error::from)
        .and_then(|s| s.parse::<Transaction>())
}

pub async fn build_from_log<R, DB>(r: R, db: DB) -> crate::Result<()>
where
    R: std::io::Read,
    DB: Repo,
{
    let transactions = BufReader::new(r)
        .lines()
        .map(line_to_transaction)
        .collect::<crate::Result<Vec<_>>>()?;

    for transaction in transactions {
        match transaction {
            Transaction::LyricDelete(id) => {
                db.delete_lyric(id).await?;
            }
            Transaction::LyricUpsert(lyric) => {
                db.upsert_lyric(lyric).await?;
            }
            Transaction::PlaylistDelete(id) => {
                db.delete_playlist(id).await?;
            }
            Transaction::PlaylistUpsert(playlist) => {
                db.upsert_playlist(playlist).await?;
            }
        }
    }
    Ok(())
}

pub fn log_to_transaction<W>(mut writer: W) -> impl FnMut(Transaction) -> crate::Result<()>
where
    W: std::io::Write,
{
    move |transaction| write(&mut writer, transaction.to_string())
}

pub fn start_log_thread<W>(
    log: W,
) -> (
    JoinHandle<crate::Result<()>>,
    std::sync::mpsc::Sender<Transaction>,
)
where
    W: std::io::Write + Send + Sync + 'static,
{
    let (log_tx, log_rx) = std::sync::mpsc::channel::<Transaction>();
    let join_handle = std::thread::spawn(move || {
        let mut write = log_to_transaction(log);
        while let Ok(request) = log_rx.recv() {
            write(request)?;
        }
        Ok::<(), crate::Error>(())
    });
    (join_handle, log_tx)
}
