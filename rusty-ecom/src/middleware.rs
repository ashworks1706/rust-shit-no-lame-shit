use axum::{
    extract::{ Request, FromRequestParts},
    http::{header, StatusCode, request::Parts},
    response::{ Response, IntoResponse},
    body::Body,
};

use crate::auth::{validate_jwt, Claims};
use crate::error::AppError;


// need ot be extracted from request headers for handlers that need aut h
pub struct AuthUser{
    pub user_id: String,
    pub email: String,
}

// implement extractor 
//
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // get authorization header
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .ok_or(AppError::Unauthorized("Missing Authorization Header".to_string()))?
            .to_str()
            .map_err(|_| AppError::Unauthorized("Invalid Authorization Header".to_string()))?;

        // check if it starts with "Bearer "
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Invalid Authorization Scheme".to_string()));
        }

        // extract token
        let token = auth_header.trim_start_matches("Bearer ").trim();

        // validate token
        let claims = validate_jwt(token).map_err(|_| AppError::Unauthorized("Invalid Token".to_string()))?;

        Ok(AuthUser {
            user_id: claims.sub,
            email: claims.email,
        })
    }
}
