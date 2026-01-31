use serde::{Serialize, Deserialize};
use sqlx::FromRow;

// Shared product data (no ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductData {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}

// Database model (with ID)
#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}

// User model (with ID)
#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)] // never send pass hash to client 
    pub password_hash: String,
    pub full_name: String,
    pub created_at: i64,
}

// User registration input
#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub full_name: String,
}

// User login input
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

// API input types (reuse ProductData)
pub type CreateProduct = ProductData;
pub type UpdateProduct = ProductData;
