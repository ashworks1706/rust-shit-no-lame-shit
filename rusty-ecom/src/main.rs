use axum::{
    routing::{get, post, put, delete},
    response::Json,
    Router,
};
use serde_json::json;
use serde_json::Value;


mod models;
mod db;
mod api;
mod error;
mod auth; 
mod middleware;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let pool = db::init_db().await.expect("Failed to create DB pool");
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async {"Hello World"}))
        // GET REQUESTS
        .route("/product/test", get(api::get::test_product::test_product))
        .route("/health", get(health_check))
        .route("/products", get(api::get::products::get_products))
        .route("/products/:id", get(api::get::product::get_product))
        // PUT REQUESTS
        .route("/products/:id", put(api::put::product::put_product))
        // DELETE REQUESTS
        .route("/products/:id", delete(api::delete::product::delete_product))
        .route("/products", delete(api::delete::products::delete_products))
        // POST REQUESTS
        .route("/products", post(api::post::product::post_product))
        .route("/products/batch", post(api::post::products::post_products))
        // AUTH REQUESTS
        .route("/auth/register", post(api::auth::register::register_user))
        .route("/auth/login", post(api::auth::login::login_user))
        .with_state(Arc::new(pool));


    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Value> {
    Json(json!({"status": "ok"}))
}


