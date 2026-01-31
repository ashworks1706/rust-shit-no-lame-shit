use axum::{extract::State, Json};
use crate::models::{Product, CreateProduct};
use crate::error::AppError;
use std::sync::Arc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn post_product(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<CreateProduct>
) -> Result<Json<Product>, AppError> {
    let id = Uuid::new_v4().to_string();
    
    sqlx::query(
         "INSERT INTO products (id, name, description, price, stock) VALUES (?, ?, ?, ?, ?)"
     )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.price)
    .bind(payload.stock)
    .execute(&*pool)
    .await?;
    
    let product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price, stock FROM products WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await?;
    
    Ok(axum::Json(product))
}
