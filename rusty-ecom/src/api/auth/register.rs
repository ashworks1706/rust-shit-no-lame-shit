use axum::{
    extract::State,
    Json,
};
use crate::models::{User, RegisterUser};
use crate::error::AppError;
use std::sync::Arc;
use sqlx::SqlitePool;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

pub async fn register_user(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<RegisterUser>
) -> Result<Json<User>, AppError> {
    
    // Check if email already exists (do this BEFORE hashing for performance)
    let existing = sqlx::query(
        "SELECT id FROM users WHERE email = ?"
    )
    .bind(&payload.email)
    .fetch_optional(&*pool)
    .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Email already registered".to_string()));
    }

    // Hash password
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AppError::InternalError("Failed to hash password".to_string()))?;

    let id = Uuid::new_v4().to_string();

    sqlx::query(
         "INSERT INTO users (id, email, password_hash, full_name, created_at) VALUES (?, ?, ?, ?, ?)"
     )
    .bind(&id)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.full_name)
    .bind(chrono::Utc::now().timestamp())
    .execute(&*pool)
    .await?;

    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, full_name, created_at FROM users WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&*pool)
    .await?;    
    
    Ok(axum::Json(user))
}
