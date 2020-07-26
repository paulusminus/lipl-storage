use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use warp::{http, Filter, Reply, Rejection};
use serde::{Serialize, Deserialize};

type Items = HashMap<String, i32>;
type Lyrics = HashMap<u32, lipl::Lyric>;

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
    id: u32,
    title: String,
}


#[derive(Clone)]
struct Store {
    grocery_list: Arc<RwLock<Items>>,
    lyric_list: Arc<RwLock<Lyrics>>,
}

impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
            lyric_list:  Arc::new(RwLock::new(get_lyrics_store()))
        }
    }
}

fn get_lyrics_store() -> Lyrics {
    let mut result: HashMap<u32, lipl::Lyric> = HashMap::new();
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

async fn update_grocery_list(
    item: Item,
    store: Store,
) -> Result<impl Reply, Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

async fn delete_grocery_list_item(
    id: Id,
    store: Store,
) -> Result<impl Reply, Rejection> {
    store.grocery_list.write().remove(&id.name);

    Ok(warp::reply::with_status(
        "Removed item from grocery list",
        http::StatusCode::OK,
    ))
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


async fn get_grocery_list(
    store: Store,
) -> Result<impl Reply, Rejection> {
    let mut result = HashMap::new();
    let r = store.grocery_list.read();

    for (key, value) in r.iter() {
        result.insert(key, value);
    }

    Ok(warp::reply::json(
        &result,
    ))
}

fn delete_json() -> impl Filter<Extract = (Id,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/*
fn print_item<T>(item: T)
where T: std::fmt::Display {
    println!("{}", item); 
}
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    /*
    let songs = lipl::get_songs("/home/paul/Documenten/lipl.data/Geheugenkoor", "txt")?;
    songs
    .iter()
    .filter(|r| r.is_ok())
    .map(|r| r.as_ref().unwrap())
    .for_each(print_item);
    */

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

    warp::serve(get_items)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
