use std::ffi::{OsStr};
use std::path::{Path, PathBuf};
use std::pin::Pin;

use async_trait::{async_trait};
use futures::{Stream, StreamExt, TryStreamExt};
use futures::future::{ready, Ready};
use tokio::fs::{read_dir, File, remove_file};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::{LinesStream, ReadDirStream};

use crate::{RepoResult};
use lipl_types::{RepoError, Uuid};

#[async_trait]
pub trait IO {
    async fn read_string(&self) -> RepoResult<String>;
    async fn read_frontmatter(&self) -> RepoResult<String>;
    async fn remove(&self) -> RepoResult<()>;
    async fn write_string(&self, s: String) -> RepoResult<()>;

    async fn get_files<'a, F>(&self, filter: F) -> RepoResult<Pin<Box<dyn Stream<Item=RepoResult<PathBuf>> + Send + 'a>>>
    where 
        F: Fn(&PathBuf) -> Ready<bool> + Send + 'a;

    fn is_dir(&self) -> RepoResult<()>;
    fn full_path(&self, id: &str, ext: &str) -> PathBuf;
    fn id(&self) -> RepoResult<Uuid>;
}

#[async_trait]
impl<P> IO for P 
where
    P:  AsRef<Path> + Send + Sync,
{
    async fn read_string(&self) -> RepoResult<String> {
        let s = tokio::fs::read_to_string(self).await?;
        Ok(s)
    }

    async fn read_frontmatter(&self) -> RepoResult<String> {
        let file = File::open(self).await?;
        let reader = BufReader::new(file).lines();
        let lines = LinesStream::new(reader).map_err(RepoError::from);
        let part: Vec<String> = 
            lines
            .map_err(RepoError::from)
            .try_skip_while(|l| ready(Ok(l.trim() != "---")))
            .try_skip_while(|l| ready(Ok(l.trim() == "---")))
            .try_take_while(|l| ready(Ok(l.trim() != "---")))
            .try_collect()
            .await?;

        Ok(part.join("\n"))
    }

    async fn remove(&self) -> RepoResult<()> {
        remove_file(self).await?;
        Ok(())
    }

    async fn write_string(&self, s: String) -> RepoResult<()> {
        tokio::fs::write(self, s).await?;
        Ok(())
    }

    async fn get_files<'a, F>(&self, filter: F) -> RepoResult<Pin<Box<dyn Stream<Item=RepoResult<PathBuf>> + Send + 'a>>>
    where 
        F: Fn(&PathBuf) -> Ready<bool> + Send + 'a,
    {
        read_dir(self)
        .await
        .map_err(RepoError::from)
        .map(
            |de| 
                ReadDirStream::new(de)
                .map_err(RepoError::from)
                .map_ok(|de| de.path())
                .try_filter(filter)
                .boxed()
        )
    }
    
    fn is_dir(&self) -> RepoResult<()> {
        if Path::new(self.as_ref()).is_dir() {
            Ok(())
        }
        else {
            Err(RepoError::CannotFindDirectory(self.as_ref().to_str().map(String::from)))
        }
    }

    fn full_path(&self, id: &str, ext: &str) -> PathBuf {
        self.as_ref()
        .join(
            format!("{}.{}", id, ext)
        )
    }

    fn id(&self) -> RepoResult<Uuid> {
        self
        .as_ref()
        .file_stem()
        .ok_or_else(|| RepoError::Filestem(self.as_ref().to_str().map(String::from)))
        .map(|fs| fs.to_string_lossy().to_string())
        .and_then(|s| s.parse::<Uuid>())
    }
}

pub fn extension_filter(s: &str) -> impl Fn(&PathBuf) -> Ready<bool> + '_ {
    |path_buf| ready(path_buf.extension() == Some(OsStr::new(s)))
}

