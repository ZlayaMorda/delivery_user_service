use actix_web::web;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use jsonwebtoken::errors::Error;
use uuid::Uuid;
use crate::AppState;
use crate::models::token::TokenClaims;

pub fn hashing_password<'a>(
    password_input: &'a str,
    salt: &'a str
) -> Result<PasswordHash<'a>, &'a str>{

    match Argon2::default()
        .hash_password(password_input.as_bytes(), salt) {
        Ok(result) => Ok(result),
        Err(_error) => Err("Error while hashing password")
    }
}

pub fn generate_jwt(user_id: &Uuid, data: & web::Data<AppState>) -> Result<String, Error> {

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(data.env.expires_in_minutes as i64) +
        Duration::days(data.env.expires_in_days as i64)).timestamp() as usize;

    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
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
        Err(error) => Err(error)
    }
}

pub fn check_password<'a>(
    password_input: &'a str,
    salt: &'a str,
    hash: &'a str
) -> Result<bool, &'a str> {
    match hashing_password(password_input, salt).expect("Cant") {
        Ok(check_hash) => {
            if check_hash == hash {
                Ok(true)
            }
            else {
                Err("Password is not right")
            }
        }
        Err(error) => Err(error)
    }
}