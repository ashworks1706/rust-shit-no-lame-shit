use axum::{extract::{State, Path}, Json};
use crate::models::Product;
use crate::error::AppError;
use std::sync::Arc;
use sqlx::SqlitePool;

pub async fn get_product(
    Path(id): Path<String>,
    State(pool): State<Arc<SqlitePool>>
) -> Result<Json<Product>, AppError> {
    let product = sqlx::query_as::<_, Product>(
         "SELECT id, name, description, price, stock FROM products WHERE id = ?"
     )
     .bind(id.clone())
     .fetch_one(&*pool)
     .await?;
    
    Ok(axum::Json(product))
}
