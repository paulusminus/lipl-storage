use parking_lot::RwLock;
use std::collections::{BTreeMap};
use std::sync::Arc;
use warp::{Filter, Reply, Rejection};
use serde::{Serialize, Deserialize};

type Lyrics = BTreeMap<i32, lipl::Lyric>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LyricSummary {
    id: i32,
    title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Lyric {
    id: i32,
    title: String,
    parts: Vec<Vec<String>>
}

#[derive(Clone)]
struct Store {
    lyric_list: Arc<RwLock<Lyrics>>,
}

impl Store {
    fn from(list: impl std::iter::Iterator<Item = lipl::Lyric>) -> Self {
        Store {
            lyric_list: Arc::new(
                RwLock::new(
                    (1..)
                    .zip(list)
                    .collect()
                )
            ),
        }
    }

    fn get_summaries(&self) -> Vec<LyricSummary> {
        self
        .lyric_list
        .read()
        .iter()
        .map(|(id, lyric)| LyricSummary { id: id.clone(), title: format!("{}", lyric.title.to_string_lossy()) })
        .collect()
    }

    fn get_lyric(&self, id: i32) -> Option<Lyric> {
        self
        .lyric_list
        .read()
        .get(&id)
        .map(|l| Lyric {
            title: l.title.to_string_lossy().to_string(), 
            parts: l.parts.clone(), 
            id: id,
        })
    }
}

fn lyric_id_from_path(path: String) -> i32 {
    path.parse::<i32>()
    .unwrap_or_default()
}

async fn get_lyric_list(store: Store) -> Result<impl Reply, Rejection> {
    Ok(
        warp::reply::json(
            &store.get_summaries()
        )
    )
}

async fn get_lyric(path: String, store: Store) -> Result<impl Reply, Rejection> {
    store
    .get_lyric(
        lyric_id_from_path(path)
    )
    .map_or_else(
        |     | Err(warp::reject::not_found()),
        |lyric| Ok(warp::reply::json(&lyric)),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];

    let store = Store::from(
        lipl::get_songs(path, "txt")
        .unwrap()
        .into_iter()
        .filter_map(|e| e.ok())   
    );
    let store_filter = warp::any().map(move || store.clone());

    let get_items = 
        warp::get()
        .and(warp::path("v1"))
        .and(warp::path("lyric"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_lyric_list);

    let get_item = 
        warp::get()
        .and(warp::path("v1"))
        .and(warp::path("lyric"))
        .and(warp::path::param())
        .and(store_filter.clone())
        .and_then(get_lyric);

    let routes = 
        get_items
        .or(get_item);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
