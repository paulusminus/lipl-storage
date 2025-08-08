use async_trait::async_trait;
use bb8_redis::redis::AsyncCommands;
use bb8_redis::{
    RedisConnectionManager,
    bb8::{Pool, PooledConnection},
    redis::{IntoConnectionInfo, cmd},
};
use futures_util::{FutureExt, TryFutureExt, future::try_join_all};
use lipl_core::{
    Error, LiplRepo, Lyric, Playlist, Result, Summary, ToRepo, Uuid, by_title,
    parts::{to_parts, to_text},
    redis_error,
};
use std::{collections::HashMap, ops::DerefMut, str::FromStr, sync::Arc};

const LYRIC: &str = "lyric";
const PLAYLIST: &str = "playlist";
const TEXT_ATTR: &str = "text";
const TITLE_ATTR: &str = "title";
const MEMBERS_ATTR: &str = "members";
const WILDCARD: &str = "*";
const SEP: &str = ":";
const LYRIC_ALL: [&str; 3] = [LYRIC, SEP, WILDCARD];
const PLAYLIST_ALL: [&str; 3] = [PLAYLIST, SEP, WILDCARD];

fn bs58_to_uuid(r: Result<Vec<String>>) -> Result<Vec<Uuid>> {
    r.and_then(|keys| {
        keys.iter()
            .map(|s| key_to_uuid(s))
            .collect::<Result<Vec<_>>>()
    })
}

fn hashmap_to_lyric(id: Uuid) -> impl Fn(HashMap<String, String>) -> Lyric {
    move |hm| Lyric {
        id,
        title: hm.get(TITLE_ATTR).cloned().unwrap_or_default(),
        parts: to_parts(hm.get(TEXT_ATTR).cloned().unwrap_or_default()),
    }
}

fn hashmap_to_summary(id: Uuid) -> impl Fn(HashMap<String, String>) -> Summary {
    move |hm| Summary {
        id,
        title: hm.get(TITLE_ATTR).cloned().unwrap_or_default(),
    }
}

fn hashmap_to_playlist(id: Uuid) -> impl Fn(Result<HashMap<String, String>>) -> Result<Playlist> {
    move |result| {
        result.and_then(|hm| {
            hm.get(MEMBERS_ATTR)
                .cloned()
                .unwrap_or_default()
                .split(' ')
                .map(|key| key.parse::<Uuid>().ok().ok_or(Error::Key(key.to_owned())))
                .collect::<Result<Vec<Uuid>>>()
                .and_then(|members| {
                    hm.get(TITLE_ATTR)
                        .ok_or(Error::Key(id.to_string()))
                        .cloned()
                        .map(|title| (members, title))
                })
                .map(|(members, title)| Playlist { id, title, members })
        })
    }
}

fn lyric_key(id: Uuid) -> String {
    format!("{LYRIC}{SEP}{id}")
}

fn playlist_key(id: Uuid) -> String {
    format!("{PLAYLIST}{SEP}{id}")
}

fn key_to_uuid(key: &str) -> Result<Uuid> {
    key.split(':')
        .collect::<Vec<&str>>()
        .get(1)
        .ok_or(Error::Key(key.to_owned()))
        .and_then(|k| k.parse::<Uuid>().ok().ok_or(Error::Key(key.to_owned())))
}

#[derive(Clone)]
pub struct RedisRepoConfig<T>
where
    T: IntoConnectionInfo,
{
    clear: bool,
    url: T,
}

impl<T> RedisRepoConfig<T>
where
    T: IntoConnectionInfo,
{
    #[allow(dead_code)]
    pub fn new(clear: bool, url: T) -> Self {
        Self { clear, url }
    }

    pub async fn to_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        let repo = RedisRepo::new(self).await?;
        Ok(Arc::new(repo))
    }
}

impl Default for RedisRepoConfig<String> {
    fn default() -> Self {
        Self {
            clear: true,
            url: "redis://127.0.0.1/".to_owned(),
        }
    }
}

impl FromStr for RedisRepoConfig<String> {
    type Err = lipl_core::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {
            clear: false,
            url: s.to_owned(),
        })
    }
}

#[async_trait]
impl<T> ToRepo for RedisRepoConfig<T>
where
    T: IntoConnectionInfo + Send + Clone,
{
    async fn to_repo(self) -> lipl_core::Result<Arc<dyn LiplRepo>> {
        let repo = RedisRepo::new(self).await?;
        Ok(Arc::new(repo))
    }
}

pub struct RedisRepo {
    pool: Pool<RedisConnectionManager>,
    delete_lyric_sha: String,
}

impl RedisRepo {
    pub async fn new<T>(config: RedisRepoConfig<T>) -> lipl_core::Result<Self>
    where
        T: IntoConnectionInfo,
    {
        let manager = bb8_redis::RedisConnectionManager::new(config.url).map_err(redis_error)?;
        let pool = bb8_redis::bb8::Pool::builder()
            .build(manager)
            .map_err(redis_error)
            .await?;

        let pool_clone = pool.clone();
        let mut connection = pool_clone.get().map_err(redis_error).await?;

        if config.clear {
            cmd("FLUSHALL")
                .query_async::<()>(connection.deref_mut())
                .map_err(redis_error)
                .await?;
        }

        let delete_lyric_sha: String = cmd("SCRIPT")
            .arg("LOAD")
            .arg(include_str!("delete_lyric.lua"))
            .query_async(connection.deref_mut())
            .map_err(redis_error)
            .await?;

        Ok(Self {
            pool,
            delete_lyric_sha,
        })
    }

    async fn delete_lyric_script(&self, id: Uuid) -> Result<()> {
        let mut connection = self.connection().await?;
        cmd("EVALSHA")
            .arg(self.delete_lyric_sha.clone())
            .arg("0")
            .arg(id.to_string())
            .query_async(connection.deref_mut())
            .map_err(redis_error)
            .map(Into::into)
            .await
    }

    async fn connection(&self) -> Result<PooledConnection<'_, RedisConnectionManager>> {
        self.pool.get().map_err(redis_error).await
    }

    async fn delete<F>(&self, id: Uuid, f: F) -> Result<()>
    where
        F: Fn(Uuid) -> String,
    {
        self.connection()
            .and_then(
                |mut connection| async move { connection.del(f(id)).map_err(redis_error).await },
            )
            .await
    }

    async fn get_summary<F>(&self, id: Uuid, key: F) -> Result<Summary>
    where
        F: Fn(Uuid) -> String,
    {
        self.connection()
            .and_then(|mut connection| async move {
                connection
                    .hgetall(key(id))
                    .map_ok(hashmap_to_summary(id))
                    .map_err(redis_error)
                    .await
            })
            .await
    }

    async fn get_keys<F, T>(&self, filter: String, convert: F) -> Result<Vec<T>>
    where
        F: Fn(Result<Vec<String>>) -> Result<Vec<T>>,
    {
        self.connection()
            .and_then(|mut connection| async move {
                connection
                    .keys::<&str, Vec<String>>(&filter)
                    .map_err(redis_error)
                    .map(convert)
                    .await
            })
            .await
    }
}

#[async_trait]
impl LiplRepo for RedisRepo {
    async fn delete_lyric(&self, id: Uuid) -> lipl_core::Result<()> {
        self.delete_lyric_script(id).err_into().await
    }

    async fn delete_playlist(&self, id: Uuid) -> lipl_core::Result<()> {
        self.delete(id, playlist_key).err_into().await
    }

    async fn get_lyric(&self, id: Uuid) -> lipl_core::Result<Lyric> {
        self.connection()
            .and_then(|mut connection| async move {
                connection
                    .hgetall(lyric_key(id))
                    .map_err(redis_error)
                    .map_ok(hashmap_to_lyric(id))
                    .await
            })
            .err_into()
            .await
    }

    async fn get_playlist(&self, id: Uuid) -> lipl_core::Result<Playlist> {
        self.connection()
            .and_then(|mut connection| async move {
                connection
                    .hgetall(playlist_key(id))
                    .map_err(redis_error)
                    .map(hashmap_to_playlist(id))
                    .await
            })
            .err_into()
            .await
    }

    async fn get_lyrics(&self) -> lipl_core::Result<Vec<Lyric>> {
        let mut lyrics = self
            .get_keys(LYRIC_ALL.concat(), bs58_to_uuid)
            .err_into()
            .and_then(|ids| try_join_all(ids.into_iter().map(|id| self.get_lyric(id))))
            .await?;
        lyrics.sort_by(by_title);
        Ok(lyrics)
    }

    async fn get_lyric_summaries(&self) -> lipl_core::Result<Vec<Summary>> {
        let mut summaries = self
            .get_keys(LYRIC_ALL.concat(), bs58_to_uuid)
            .and_then(|ids| try_join_all(ids.into_iter().map(|id| self.get_summary(id, lyric_key))))
            .await?;
        summaries.sort_by(by_title);
        Ok(summaries)
    }

    async fn get_playlists(&self) -> lipl_core::Result<Vec<Playlist>> {
        let mut playlists = self
            .get_keys(PLAYLIST_ALL.concat(), bs58_to_uuid)
            .err_into()
            .and_then(|ids| try_join_all(ids.into_iter().map(|id| self.get_playlist(id))))
            .await?;
        playlists.sort_by(by_title);
        Ok(playlists)
    }

    async fn get_playlist_summaries(&self) -> lipl_core::Result<Vec<Summary>> {
        let mut summaries = self
            .get_keys(PLAYLIST_ALL.concat(), bs58_to_uuid)
            .and_then(|ids| {
                try_join_all(
                    ids.into_iter()
                        .map(|id| self.get_summary(id, playlist_key).err_into()),
                )
            })
            .await?;
        summaries.sort_by(by_title);
        Ok(summaries)
    }

    async fn upsert_lyric(&self, lyric: Lyric) -> lipl_core::Result<Lyric> {
        self.connection()
            .and_then(|mut connection| async move {
                connection
                    .hset_multiple::<String, String, String, ()>(
                        lyric_key(lyric.id),
                        &[
                            (TITLE_ATTR.to_owned(), lyric.title.clone()),
                            (TEXT_ATTR.to_owned(), to_text(&lyric.parts)),
                        ],
                    )
                    .map_ok(|_| lyric)
                    .map_err(redis_error)
                    .await
            })
            .err_into()
            .await
    }

    async fn upsert_playlist(&self, playlist: Playlist) -> lipl_core::Result<Playlist> {
        self.connection()
            .and_then(|mut connection| async move {
                connection
                    .hset_multiple::<String, String, String, ()>(
                        playlist_key(playlist.id),
                        &[
                            (TITLE_ATTR.to_owned(), playlist.title.clone()),
                            (
                                MEMBERS_ATTR.to_owned(),
                                playlist
                                    .members
                                    .iter()
                                    .map(|id| id.to_string())
                                    .collect::<Vec<_>>()
                                    .join(" "),
                            ),
                        ],
                    )
                    .map_ok(|_| playlist)
                    .map_err(redis_error)
                    .await
            })
            .err_into()
            .await
    }

    async fn stop(&self) -> lipl_core::Result<()> {
        Ok(())
    }
}
