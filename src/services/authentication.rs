use std::ops::Add;
use std::str;
use actix_web::web;
use sha3::{Digest, Sha3_256};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;
use crate::AppState;
use crate::models::token::TokenClaims;
use crate::utils::errors::AuthorizationError;


pub fn hashing_password<'a>(
    password_input: &'a str,
    salt: &'a str
) -> String {

    let data = salt.to_string().add(password_input);
    format!("{:x}", Sha3_256::digest(data))
}

pub fn generate_jwt(user_id: &Uuid, user_role: &str, data: & web::Data<AppState>) -> Result<String, AuthorizationError> {

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(data.env.expires_in_minutes as i64) +
        Duration::days(data.env.expires_in_days as i64)).timestamp() as usize;

    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        rol: user_role.to_string(),
        exp,
        iat,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&data.env.secret.as_bytes()),
    );

    match token {
        Ok(result) => Ok(result),
        Err(error) => Err(AuthorizationError::FailedToGenerateJWT(error.to_string()))
    }
}

pub fn check_password(
    password_input: &str,
    salt: &str,
    hash: &str
) -> Result<(), AuthorizationError> {
    if hashing_password(password_input, salt) == hash {
        Ok(())
    }
    else {
        Err(AuthorizationError::InvalidPassword("Invalid password".to_string()))
    }
}