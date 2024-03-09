use warp::{http, Filter};

use crate::store::{Id, Item, Store}; // This is how you use the Store and Item structs from the store module

pub async fn add_grocery_list_item(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let r = store.grocery_list.read();
    Ok(warp::reply::json(&*r))
}
pub async fn update_grocery_list(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

pub async fn delete_grocery_list_item(
    id: Id,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().remove(&id.name);

    Ok(warp::reply::with_status(
        "Removed item from grocery list",
        http::StatusCode::OK,
    ))
}

pub async fn get_grocery_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.grocery_list.read();
    Ok(warp::reply::json(&*result))
}

pub fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
