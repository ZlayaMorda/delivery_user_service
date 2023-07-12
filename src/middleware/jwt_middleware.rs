use std::future::{ready, Ready};
use std::rc::Rc;
//use actix::fut::ok;

use actix_web::error::{ErrorUnauthorized};
use actix_web::{Error, HttpMessage};
use actix_web::{http};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use futures::future::LocalBoxFuture;

use crate::AppState;
use crate::models::token::TokenClaims;
use crate::models::users::MiddlewareUserInfo;


pub struct FactoryCheckJWT;

impl<S, B> Transform<S, ServiceRequest> for FactoryCheckJWT
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckJWTMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {

        ready(Ok(CheckJWTMiddleware { service: Rc::new(service) }))
    }
}

pub struct CheckJWTMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckJWTMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let token = req.headers()
                .get(http::header::AUTHORIZATION)
                .map(|h| h.to_str().
                    expect("Error while converting token bytes to string").to_string());

        let service_clone = self.service.clone();

        Box::pin(async move {
            let token_str = match token {
                Some(result) => result,
                None => return Err(ErrorUnauthorized("Unauthorized, provide token"))
            };

            let data= match req.app_data::<Data<AppState>>()
            {
                Some(result) => result,
                None => return Err(ErrorUnauthorized("Unauthorized, can't get app data")),
            };

            match decode::<TokenClaims>(
                token_str.as_str(),
                &DecodingKey::from_secret(data.env.secret.as_ref()),
                &Validation::default(),
            ) {
                Ok(c) => {
                    let iat = Utc::now().timestamp() as usize;
                    if iat >= c.claims.exp {
                        return Err(ErrorUnauthorized("Unauthorized, invalid token"));
                    }

                    let temp = c.claims;
                    let user_info = MiddlewareUserInfo {
                        user_uuid: temp.sub.parse().expect("Cant convert string to UUid"),
                        role: temp.rol,
                    };

                    req.extensions_mut().insert(user_info);
                },
                Err(_) => { return Err(ErrorUnauthorized("Unauthorized, invalid token")); }
            };


            let fut = service_clone.call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
}