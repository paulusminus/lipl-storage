/*
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
*/

/*
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
*/

/*
fn delete_json() -> impl Filter<Extract = (Id,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
*/

/*
fn print_item<T>(item: T)
where T: std::fmt::Display {
    println!("{}", item); 
}
*/

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
    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list);
    */

