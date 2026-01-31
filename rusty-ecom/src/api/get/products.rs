use axum::{extract::State, Json};
use crate::models::Product;
use std::sync::Arc;
use sqlx::SqlitePool;
use crate::error::AppError;
pub async fn get_products(State(pool): State<Arc<SqlitePool>>) -> Result<Json<Vec<Product>>, AppError> {
    let products = sqlx::query_as::<_, Product>(
         "SELECT * FROM products"
     )
     .fetch_all(&*pool)
     .await?;
    
    Ok(axum::Json(products))
}
