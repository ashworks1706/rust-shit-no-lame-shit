use axum::{extract::{State, Path}, Json};
use crate::error::AppError;
use std::sync::Arc;
use sqlx::SqlitePool;
use serde_json::{json, Value};
use crate::middleware::AuthUser;

pub async fn delete_product(
    auth: AuthUser, 
    Path(id): Path<String>,
    State(pool): State<Arc<SqlitePool>>
) -> Result<Json<Value>, AppError> {
    let result = sqlx::query(
         "DELETE FROM products WHERE id = ?"
    )
    .bind(&id)
    .execute(&*pool)
    .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Product with id {} not found", id)));
    }
    
    Ok(axum::Json(json!({"message": "Product deleted successfully", "id": id, "deleted_by": auth.email})))
}
