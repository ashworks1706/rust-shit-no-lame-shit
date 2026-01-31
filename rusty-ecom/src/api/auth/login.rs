use axum::{
    extract::State,
    Json,
};
use crate::models::{User, LoginUser};
use crate::error::AppError;
use crate::auth::create_jwt;
use std::sync::Arc;
use sqlx::SqlitePool;
use bcrypt::verify;
use serde_json::{json, Value};

pub async fn login_user(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<LoginUser>
) -> Result<Json<Value>, AppError> {
    
    // Find user by email
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, full_name, created_at FROM users WHERE email = ?"
    )
    .bind(&payload.email)
    .fetch_optional(&*pool)
    .await?
    .ok_or_else(|| AppError::BadRequest("Invalid email or password".to_string()))?;

    // Verify password
    let valid_pass = verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::InternalError("Failed to verify password".to_string()))?;

    if !valid_pass {
        return Err(AppError::BadRequest("Invalid email or password".to_string()));
    }

    // Generate JWT token
    let token = create_jwt(user.id.clone(), user.email.clone())
        .map_err(|_| AppError::InternalError("Failed to create token".to_string()))?;

    // Return success with token
    Ok(axum::Json(json!({
        "message": "Login successful",
        "token": token,
        "user": {
            "id": user.id,
            "email": user.email,
            "full_name": user.full_name,
            "created_at": user.created_at
        }
    })))
}
