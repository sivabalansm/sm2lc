mod item_list;
mod item;
mod db;
use axum::{Router, routing::get, routing::post, response::Json};
use std::sync::Arc;
use crate::{db::DB, item::Item};

const PORT : u64 = 3030;

// handlers for db stuff
async fn handler_update_item() {}
async fn handler_put_item() {}
async fn handler_get_all_items(db : &DB) -> Json<Vec<Item>>{
    Json(db.get_all_items().await.unwrap())
}


#[tokio::main]
async fn main() {
    let db = Arc::new(DB::new());
    let items = db.get_all_items().await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/update_item", post(handler_update_item))
        .route("/put_item", post(handler_put_item))
        .route("/get_all_items", get(async move || handler_get_all_items(&(db.clone())).await ));


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



