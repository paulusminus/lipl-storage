use std::sync::Arc;

#[cfg(feature = "fs")]
use futures_util::future::BoxFuture;
use lipl_core::{LiplRepo, Lyric, Playlist, Summary};

#[cfg(feature = "memory")]
use lipl_storage_memory::MemoryRepo;
#[cfg(feature = "postgres")]
use lipl_storage_postgres::PostgresConnectionPool;

/// Configure repo from environment
use crate::{Error, Result, ToRepo};

pub enum RepoType {
    #[cfg(feature = "postgres")]
    Postgres(String),
    #[cfg(feature = "memory")]
    Memory(bool),
    #[cfg(feature = "fs")]
    Fs(String),
    #[cfg(feature = "redis")]
    Redis(String),
}

#[derive(Clone)]
pub enum ServerRepo {
    #[cfg(feature = "postgres")]
    Postgres(PostgresConnectionPool),
    #[cfg(feature = "memory")]
    Memory(MemoryRepo),
    #[cfg(feature = "fs")]
    Fs(lipl_storage_fs::FileRepo),
    #[cfg(feature = "redis")]
    Redis(lipl_storage_redis::redis_repo::RedisRepo),
}

impl LiplRepo for ServerRepo {
    fn get_lyrics(&self) -> BoxFuture<'_, lipl_core::Result<Vec<Lyric>>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.get_lyrics(),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.get_lyrics(),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.get_lyrics(),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.get_lyrics(),
        }
    }

    fn get_lyric_summaries(&self) -> BoxFuture<'_, lipl_core::Result<Vec<Summary>>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.get_lyric_summaries(),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.get_lyric_summaries(),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.get_lyric_summaries(),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.get_lyric_summaries(),
        }
    }

    fn get_lyric(&self, id: lipl_core::Uuid) -> BoxFuture<'_, lipl_core::Result<Lyric>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.get_lyric(id),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.get_lyric(id),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.get_lyric(id),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.get_lyric(id),
        }
    }

    fn upsert_lyric(&self, lyric: Lyric) -> BoxFuture<'_, lipl_core::Result<Lyric>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.upsert_lyric(lyric),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.upsert_lyric(lyric),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.upsert_lyric(lyric),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.upsert_lyric(lyric),
        }
    }

    fn delete_lyric(&self, id: lipl_core::Uuid) -> BoxFuture<'_, lipl_core::Result<()>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.delete_lyric(id),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.delete_lyric(id),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.delete_lyric(id),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.delete_lyric(id),
        }
    }

    fn get_playlists(&self) -> BoxFuture<'_, lipl_core::Result<Vec<lipl_core::Playlist>>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.get_playlists(),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.get_playlists(),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.get_playlists(),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.get_playlists(),
        }
    }

    fn get_playlist_summaries(&self) -> BoxFuture<'_, lipl_core::Result<Vec<lipl_core::Summary>>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.get_playlist_summaries(),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.get_playlist_summaries(),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.get_playlist_summaries(),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.get_playlist_summaries(),
        }
    }

    fn get_playlist(&self, id: lipl_core::Uuid) -> BoxFuture<'_, lipl_core::Result<Playlist>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.get_playlist(id),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.get_playlist(id),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.get_playlist(id),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.get_playlist(id),
        }
    }

    fn upsert_playlist(&self, playlist: Playlist) -> BoxFuture<'_, lipl_core::Result<Playlist>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.upsert_playlist(playlist),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.upsert_playlist(playlist),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.upsert_playlist(playlist),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.upsert_playlist(playlist),
        }
    }

    fn delete_playlist(&self, id: lipl_core::Uuid) -> BoxFuture<'_, lipl_core::Result<()>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.delete_playlist(id),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.delete_playlist(id),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.delete_playlist(id),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.delete_playlist(id),
        }
    }

    fn stop(&self) -> BoxFuture<'_, lipl_core::Result<()>> {
        match self {
            #[cfg(feature = "postgres")]
            Self::Postgres(pool) => pool.stop(),
            #[cfg(feature = "memory")]
            Self::Memory(repo) => repo.stop(),
            #[cfg(feature = "fs")]
            Self::Fs(repo) => repo.stop(),
            #[cfg(feature = "redis")]
            Self::Redis(repo) => repo.stop(),
        }
    }
}

// impl ToRepo for RepoType {
//     type Repo = ServerRepo;
//     fn to_repo(self) -> lipl_core::Result<Self::Repo> {
//         match self {
//             #[cfg(feature = "postgres")]
//             Self::Postgres(connection) => {
//                 lipl_storage_postgres::connection_pool(&connection).map(ServerRepo::Postgres)
//             }
//             #[cfg(feature = "memory")]
//             Self::Memory(include_sample) => lipl_storage_memory::MemoryRepoConfig {
//                 sample_data: include_sample,
//                 transaction_log: None,
//             }
//             .to_repo()
//             .map_ok(ServerRepo::Memory)
//             .boxed(),
//             #[cfg(feature = "fs")]
//             Self::Fs(data_dir) => lipl_storage_fs::FileRepoConfig { path: data_dir }
//                 .to_repo()
//                 .map_ok(ServerRepo::Fs)
//                 .boxed(),
//             #[cfg(feature = "redis")]
//             Self::Redis(connection) => {
//                 lipl_storage_redis::redis_repo::RedisRepoConfig::new(false, connection)
//                     .to_repo()
//                     .map_ok(ServerRepo::Redis)
//                     .boxed()
//             }
//         }
//     }
// }

fn var(key: &'static str) -> Result<String> {
    std::env::var(key).map_err(Error::from)
}

#[cfg(feature = "memory")]
fn include_sample_data() -> Result<bool> {
    var("LIPL_STORAGE_MEMORY_SAMPLE").and_then(|s| s.parse::<bool>().map_err(Error::from))
}

pub fn repo_type() -> Result<Arc<dyn LiplRepo>> {
    var("LIPL_STORAGE_REPO_TYPE").and_then(|s| {
        let repo_type = s.trim().to_lowercase();
        let r = repo_type.as_str();

        #[cfg(feature = "postgres")]
        if r == "postgres" {
            use lipl_storage_postgres::connection_pool;
            let s = postgres_connection()?;
            let pool = connection_pool(&s)?;
            return Ok(Arc::new(pool) as Arc<dyn LiplRepo>);
        }

        #[cfg(feature = "fs")]
        if r == "fs" {
            use lipl_storage_fs::FileRepoConfig;
            let s = file_path();
            let repo = s.parse::<FileRepoConfig>()?.to_repo()?;
            return Ok(Arc::new(repo));
        }

        #[cfg(feature = "memory")]
        if r == "memory" {
            use lipl_storage_memory::MemoryRepoConfig;
            let s = include_sample_data()?;
            let repo = MemoryRepoConfig {
                sample_data: s,
                transaction_log: None,
            }
            .to_repo()?;
            return Ok(Arc::new(repo));
        }

        #[cfg(feature = "redis")]
        if r == "redis" {
            use lipl_storage_redis::RedisRepoConfig;
            let s = redis_connection()?;
            let repo = s.parse::<RedisRepoConfig<_>>()?.to_repo()?;
            return Ok(Arc::new(repo));
        }

        Err(Error::InvalidConfiguration)
    })
}

#[cfg(feature = "postgres")]
fn postgres_connection() -> Result<String> {
    var("LIPL_STORAGE_POSTGRES_CONNECTION")
}

#[cfg(feature = "redis")]
fn redis_connection() -> Result<String> {
    var("LIPL_STORAGE_REDIS_CONNECTION")
}

#[cfg(feature = "fs")]
fn file_path() -> String {
    var("LIPL_STORAGE_FS_DIR").unwrap_or(".".to_owned())
}
