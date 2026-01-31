use axum::{extract::State, Json};
use crate::error::AppError;
use std::sync::Arc;
use sqlx::SqlitePool;
use serde_json::{json, Value};

pub async fn delete_products(
    State(pool): State<Arc<SqlitePool>>,
    Json(ids): Json<Vec<String>>
) -> Result<Json<Value>, AppError> {
    let mut deleted_count = 0;
    
    for id in ids {
        let result = sqlx::query(
             "DELETE FROM products WHERE id = ?"
        )
        .bind(&id)
        .execute(&*pool)
        .await?;
        
        deleted_count += result.rows_affected();
    }
    
    Ok(axum::Json(json!({
        "message": "Products deleted successfully",
        "deleted_count": deleted_count
    })))
}
