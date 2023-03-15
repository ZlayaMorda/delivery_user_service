use core::fmt;
use std::future::{ready, Ready};
use actix::fut::ok;

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError, Error};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

use crate::AppState;
use crate::models::token::TokenClaims;


#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct JwtMiddleware {
    pub user_id: String,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token = req.headers()
            .get(http::header::AUTHORIZATION)
            .map(|h| h.to_str().unwrap().split_at(7).1.to_string());

        if token.is_none() {
            let json_error = ErrorResponse {
                status: "Fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(data.env.secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        let user_id = claims.sub.to_string();

        req.extensions_mut()
            .insert::<String>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}

pub struct CheckJWT;

impl<S> Transform<S, ServiceRequest> for CheckJWT
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = CheckJWTMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckJWTMiddleware { service })
    }
}

pub struct CheckJWTMiddleware<S> {
    service: S,
}