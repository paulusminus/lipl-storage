use std::ffi::{OsStr};
use std::path::{Path, PathBuf};
use std::pin::Pin;

use async_trait::{async_trait};
use futures::{Stream, StreamExt, TryStreamExt, TryFutureExt};
use futures::future::{ready, Ready};
use tokio::fs::{read_dir, File, remove_file};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::{LinesStream, ReadDirStream};

use crate::error::{FileRepoError};
use lipl_core::{Uuid};

type Result<T> = std::result::Result<T, FileRepoError>;

#[async_trait]
pub trait IO {
    async fn read_string(&self) -> Result<String>;
    async fn read_frontmatter(&self) -> Result<String>;
    async fn remove(&self) -> Result<()>;
    async fn write_string(&self, s: String) -> Result<()>;

    async fn get_files<'a, F>(&self, filter: F) -> Result<Pin<Box<dyn Stream<Item=Result<PathBuf>> + Send + 'a>>>
    where 
        F: Fn(&PathBuf) -> Ready<bool> + Send + 'a;

    fn is_dir(&self) -> Result<()>;
    fn full_path(&self, id: &str, ext: &str) -> PathBuf;
    fn id(&self) -> Result<Uuid>;
}

#[async_trait]
impl<P> IO for P 
where
    P:  AsRef<Path> + Send + Sync,
{
    async fn read_string(&self) -> Result<String> {
        let s = tokio::fs::read_to_string(self).await?;
        Ok(s)
    }

    async fn read_frontmatter(&self) -> Result<String> {
        File::open(self)
        .map_ok(BufReader::new)
        .map_ok(|buf_reader| buf_reader.lines())
        .map_ok(LinesStream::new)
        .and_then(|stream|
            stream
            .try_skip_while(|l| ready(Ok(l.trim() != "---")))
            .try_skip_while(|l| ready(Ok(l.trim() == "---")))
            .try_take_while(|l| ready(Ok(l.trim() != "---")))
            .try_collect::<Vec<String>>()
        )
        .map_err(Into::into)
        .map_ok(|parts| parts.join("\n"))
        .await
    }

    async fn remove(&self) -> Result<()> {
        remove_file(self).await?;
        Ok(())
    }

    async fn write_string(&self, s: String) -> Result<()> {
        tokio::fs::write(self, s)
        .map_err(Into::into)
        .await
    }

    async fn get_files<'a, F>(&self, filter: F) -> Result<Pin<Box<dyn Stream<Item=Result<PathBuf>> + Send + 'a>>>
    where 
        F: Fn(&PathBuf) -> Ready<bool> + Send + 'a,
    {
        read_dir(self)
        .map_err(Into::into)
        .map_ok(
            |de| 
                ReadDirStream::new(de)
                .map_err(Into::into)
                .map_ok(|de| de.path())
                .try_filter(filter)
                .boxed()
        )
        .await
    }
    
    fn is_dir(&self) -> Result<()> {
        if Path::new(self.as_ref()).is_dir() {
            Ok(())
        }
        else {
            Err(FileRepoError::CannotFindDirectory(self.as_ref().to_str().map(String::from)))
        }
    }

    fn full_path(&self, id: &str, ext: &str) -> PathBuf {
        self.as_ref()
        .join(
            format!("{}.{}", id, ext)
        )
    }

    fn id(&self) -> Result<Uuid> {
        self
        .as_ref()
        .file_stem()
        .ok_or_else(|| FileRepoError::Filestem(self.as_ref().to_str().map(String::from)))
        .map(|fs| fs.to_string_lossy().to_string())
        .and_then(|s| s.parse::<Uuid>().map_err(|_| FileRepoError::Parse(format!("Uuid from {}", self.as_ref().to_string_lossy()))))
    }
}

pub fn extension_filter(s: &str) -> impl Fn(&PathBuf) -> Ready<bool> + '_ {
    |path_buf| ready(path_buf.extension() == Some(OsStr::new(s)))
}

