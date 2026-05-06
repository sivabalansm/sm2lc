mod item_list;
mod item;
mod db;
use axum::{Router, routing::get, routing::post, response::Json};
use serde_json::json;
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
    let db = DB::new();
    let items = db.get_all_items().await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/update_item", post(handler_update_item))
        .route("/put_item", post(handler_put_item))
        .route("/get_all_items", get(|| async { handler_get_all_items(&db) }));


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



