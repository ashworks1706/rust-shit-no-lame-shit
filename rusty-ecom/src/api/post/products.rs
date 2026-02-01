use axum::{extract::State, Json};
use crate::models::{Product, CreateProduct};
use crate::error::AppError;
use crate::middleware::AuthUser;
use std::sync::Arc;
use sqlx::SqlitePool;
use uuid::Uuid;

// Post batch of products
pub async fn post_products(
    auth: AuthUser,
    State(pool): State<Arc<SqlitePool>>, 
    Json(products): Json<Vec<CreateProduct>>
) -> Result<Json<Vec<Product>>, AppError> {
    let mut created_products = Vec::new();
    
    for product in products {
        let id = Uuid::new_v4().to_string();
        
        sqlx::query(
            "INSERT INTO products (id, name, description, price, stock) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&product.name)
        .bind(&product.description)
        .bind(product.price)
        .bind(product.stock)
        .execute(&*pool)
        .await?;
        
        let inserted = sqlx::query_as::<_, Product>(
            "SELECT id, name, description, price, stock FROM products WHERE id = ?"
        )
        .bind(&id)
        .fetch_one(&*pool)
        .await?;
        
        created_products.push(inserted);
    }
    
    Ok(Json(created_products))
}
