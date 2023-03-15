use std::ops::Add;
use std::str;
use actix_web::web;
use sha3::{Digest, Sha3_256};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use jsonwebtoken::errors::Error;
use uuid::Uuid;
use crate::AppState;
use crate::models::token::TokenClaims;


pub fn hashing_password<'a>(
    password_input: &'a str,
    salt: &'a str
) -> String {

    let data = salt.to_string().add(password_input);
    format!("{:x}", Sha3_256::digest(data))
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
) -> Result<(), &'a str> {
    if hashing_password(password_input, salt) == hash {
        Ok(())
    }
    else {
        Err("Password is not right")
    }
}