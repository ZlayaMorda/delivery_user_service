use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::error::{ErrorUnauthorized};
use actix_web::{Error, HttpMessage};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::LocalBoxFuture;

use crate::models::users::MiddlewareUserInfo;


pub struct FactoryPermissionCheck {
    permission: Rc<Vec<String>>,
}

impl FactoryPermissionCheck {
    pub fn new(permission: Vec<String>) -> Self {
        FactoryPermissionCheck {
            permission: Rc::new(permission),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for FactoryPermissionCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckPermissionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckPermissionMiddleware {
            permission: self.permission.clone(),
            service: Rc::new(service),
        }))
    }
}

pub struct CheckPermissionMiddleware<S> {
    permission: Rc<Vec<String>>,
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckPermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let permission_clone = self.permission.clone();
        let service_clone = self.service.clone();

        Box::pin(async move {
            match req.extensions_mut().get::<MiddlewareUserInfo>() {
                None => { return Err(ErrorUnauthorized("Permission denied")); }
                Some(result) => {
                    if !permission_clone.contains(& result.role) {
                        return Err(ErrorUnauthorized("Permission denied"));
                    }
                }
            };

            let fut = service_clone.call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
}