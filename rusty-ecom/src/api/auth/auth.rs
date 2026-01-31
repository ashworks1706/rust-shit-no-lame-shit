use serde::{Deserialize, Serialize};
use jsonwebtoken::{EncodingKey, DecodingKey, Header, Validation, encode, decode, TokenData, errors::Result as JwtResult};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    pub sub:String,
    pub email: String,
    pub exp: usize,
} 

// generate jwt token for sessions 
pub fn create_jwt(user_id: String, email: String) -> Result<String, jsonwebtoken::errors::Error>{

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
        &EncodingKey::from_secret("secret_key".as_ref()),
    )?;
    Ok(token)
}

pub fn validate_jwt(token: &str) -> Result<Claims,  jsonwebtoken::errors::Error>{
    let token_data: TokenData<Claims> = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret_key".as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}


