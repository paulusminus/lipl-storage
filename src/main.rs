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
    fn new() -> Self {
        Store {
            lyric_list:  Arc::new(RwLock::new(get_lyrics_store())),
        }
    }
    fn get_summaries(&self) -> Vec<LyricSummary> {
        let mut result: Vec<LyricSummary> = vec!();
        for (id, lyric) in self.lyric_list.read().iter() {
            result.push(LyricSummary { id: id.clone(), title: format!("{}", lyric.title.to_string_lossy()) });
        }
        result
    }
    fn get_lyric(&self, id: i32) -> Option<Lyric> {
        self.lyric_list.read().get(&id).map(|l| Lyric {
            title: l.title.to_string_lossy().to_string(), 
            parts: l.parts.clone(), 
            id: id,
        })
    }
}

fn get_lyrics_store() -> Lyrics {
    let mut result: BTreeMap<i32, lipl::Lyric> = BTreeMap::new();
    let mut count = 0;

    let entries = lipl::get_songs("/home/paul/Documenten/lipl.data/Geheugenkoor", "txt").unwrap();
    for entry in entries {
        match entry {
            Ok(e) => { 
                count += 1;
                result.insert(count, e);
            },
            Err(_) => {},
        }
    }

    result
}

fn lyric_id_from_path(path: String) -> i32 {
    path.parse::<i32>().unwrap_or_default()
}

async fn get_lyric_list(store: Store) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(
        &store.get_summaries()
    ))
}

async fn get_lyric(path: String, store: Store) -> Result<impl Reply, Rejection> {
    store
    .get_lyric(
        lyric_id_from_path(path)
    )
    .map_or_else(
        || Err(warp::reject::not_found()),
        |lyric| Ok(warp::reply::json(&lyric)),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = Store::new();
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
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
