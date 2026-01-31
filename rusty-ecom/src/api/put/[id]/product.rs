use axum::{extract::{State, Path}, Json};
use crate::models::{Product, UpdateProduct};
use crate::error::AppError;
use std::sync::Arc;
use sqlx::SqlitePool;

pub async fn put_product(
    Path(id): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<UpdateProduct>
) -> Result<Json<Product>, AppError> {
    sqlx::query(
            "UPDATE products SET name = ?, description = ?, price = ?, stock = ? WHERE id = ?"
        )
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(payload.price)
        .bind(payload.stock)
        .bind(&id)
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
