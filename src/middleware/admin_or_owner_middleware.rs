use std::future::{ready, Ready};
use std::rc::Rc;

use actix_web::error::{ErrorUnauthorized};
use actix_web::{Error, HttpMessage};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures::future::LocalBoxFuture;

use crate::models::users::MiddlewareUserInfo;


pub struct FactoryAdminOrOwnerId;

impl<S, B> Transform<S, ServiceRequest> for FactoryAdminOrOwnerId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AdminOrOwnerIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminOrOwnerIdMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AdminOrOwnerIdMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AdminOrOwnerIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let service_clone = self.service.clone();

        Box::pin(async move {
            match req.extensions_mut().get::<MiddlewareUserInfo>() {
                None => { return Err(ErrorUnauthorized("Permission denied")); }
                Some(result) => {
                    if result.role != "admin" {

                        match req.match_info().get("id") {
                            None => { return Err(ErrorUnauthorized("Do not set id in uri")); }
                            Some(id) => {
                                if result.user_uuid.to_string() != id {
                                    return Err(ErrorUnauthorized("Permission denied"));
                                }
                            }
                        }
                    }
                }
            };

            let fut = service_clone.call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
}//8559b330-9e97-4b99-b055-45c88878be71