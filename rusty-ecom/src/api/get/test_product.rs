use axum::{extract::State, Json};
use crate::models::Product;
use crate::error::AppError;
use std::sync::Arc;
use sqlx::SqlitePool;

pub async fn test_product(State(pool): State<Arc<SqlitePool>>) -> Result<Json<Product>, AppError> {
    let product = sqlx::query_as::<_, Product>(
         "SELECT id, name, description, price, stock FROM products WHERE id = ?"
     )
     .bind("1")
     .fetch_one(&*pool)
     .await?;
    
    Ok(axum::Json(product))
}
