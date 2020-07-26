use parking_lot::RwLock;
use std::collections::{BTreeMap};
use std::sync::Arc;
use warp::{Filter, Reply, Rejection};
use serde::{Serialize, Deserialize};

type Lyrics = BTreeMap<i32, lipl::Lyric>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

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


async fn get_lyric_list(store: Store) -> Result<impl Reply, Rejection> {
    let mut result: Vec<LyricSummary> = vec!();
    let r = store.lyric_list.read();

    for (id, lyric) in r.iter() {
        result.push(LyricSummary { id: id.clone(), title: format!("{}", lyric.title.to_string_lossy()) });
    }

    Ok(warp::reply::json(
        &result,
    ))
}

async fn get_lyric(path: String, store: Store) -> Result<impl Reply, Rejection> {
    let lyric_id = path.parse::<i32>().unwrap_or_default();
    
    match store.lyric_list.read().get(&lyric_id) {
        Some(item) => Ok(
            warp::reply::json(
                &Lyric { 
                    title: item.title.to_string_lossy().to_string(), 
                    parts: item.parts.clone(), 
                    id: lyric_id 
                }
            )
        ),
        None => Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    /*
    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);
    */

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("lyric"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_lyric_list);

    let get_item = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("lyric"))
        .and(warp::path::param())
        .and(store_filter.clone())
        .and_then(get_lyric);

    /*
    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);

    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);
    */

    /*
    let routes = add_items
        .or(get_items)
        .or(delete_item)
        .or(update_item);
    */

    let routes = get_items
    .or(get_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
