use std::{io::{BufReader, BufRead}, thread::JoinHandle, fs::{File}};

use chrono::SecondsFormat;
use serde::{Deserialize, Serialize};
use crate::{Lyric, Playlist, Summary, Uuid};

pub type ResultSender<T> = futures::channel::oneshot::Sender<crate::Result<T>>;
pub type OptionalTransaction = Option<Transaction>;
type LogRecord = (String, Transaction);

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

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&(now(), self)).unwrap())
    }
}

impl std::str::FromStr for Transaction {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str::<LogRecord>(s)
            .map(|l| l.1)
            .map_err(|e| crate::Error::Json(Box::new(e)))
    }
}

impl From<&Request> for OptionalTransaction {
    fn from(request: &Request) -> Self {
        match request {
            Request::LyricDelete(uuid, _) => Some(Transaction::LyricDelete(*uuid)),
            Request::LyricPost(lyric, _) => Some(Transaction::LyricUpsert(lyric.clone())),
            Request::PlaylistDelete(uuid, _) => Some(Transaction::PlaylistDelete(*uuid)),
            Request::PlaylistPost(playlist, _) => Some(Transaction::PlaylistUpsert(playlist.clone())),
            _ => None,
        }
    }
}

fn now() -> String {
    chrono::Utc::now()
        .to_rfc3339_opts(SecondsFormat::Micros, true)
}

fn write<W>(w: &mut W, json: String) -> crate::Result<()>
where
    W: std::io::Write,
{
    w.write_fmt(format_args!("{}\n", json))?;
    w.flush()?;
    Ok(())
}

fn line_to_transaction(line: std::io::Result<String>) -> crate::Result<Transaction> {
    line.map_err(crate::Error::from)
        .and_then(|s| s.parse::<Transaction>())
}

pub fn transactions_from_log<R>(r: R) -> impl Iterator<Item = crate::Result<Transaction>>
where
    R: std::io::Read,
{
    BufReader::new(r)
        .lines()
        .map(line_to_transaction)
}

pub fn log_to_transaction<W>(mut writer: W) -> impl FnMut(Transaction) -> crate::Result<()>
where
    W: std::io::Write,
{
    move |transaction| {
        write(&mut writer, transaction.to_string())
    }
}

pub fn start_log_thread(log: File) -> (JoinHandle<crate::Result<()>>, std::sync::mpsc::Sender<Transaction>) {
    let (log_tx, log_rx) = std::sync::mpsc::channel::<Transaction>();
    let join_handle = std::thread::spawn(move || {
        let mut write = log_to_transaction(log);
        while let Ok(request) = log_rx.recv() {
            write(request)?;
        };
        Ok::<(), crate::Error>(())
    });
    (join_handle, log_tx)
}