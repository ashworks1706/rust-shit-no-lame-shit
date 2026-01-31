use serde::{Deserialize, Serialize};
use jsonwebtoken::{EncodingKey, DecodingKey, Header, Validation, encode, decode, TokenData};

const SECRET: &[u8] = b"your_super_secret_jwt_key_min_32_chars_change_in_production";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // user_id
    pub email: String,
    pub exp: usize,       // expiration timestamp
}

// Generate JWT token for sessions
pub fn create_jwt(user_id: String, email: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )?;
    
    Ok(token)
}

// Validate JWT and extract claims
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data: TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;
    
    Ok(token_data.claims)
}

