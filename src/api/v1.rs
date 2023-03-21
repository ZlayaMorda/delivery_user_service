use actix_web::web;

use crate::handlers::{
    authentication::{register_user_handler, show_users, login_user_handler}
};
use crate::middleware::admin_or_owner_middleware::FactoryAdminOrOwnerId;
use crate::middleware::jwt_middleware::FactoryCheckJWT;
use crate::middleware::permission_middleware::FactoryPermissionCheck;


pub fn users_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/v1")
        .service(
            // Test route with test permission
            web::resource("/show/{id}").route(web::get().to(show_users))
                .wrap(FactoryAdminOrOwnerId)
                .wrap(FactoryPermissionCheck::new(vec!["user".to_string(), "admin".to_string()]))
                .wrap(FactoryCheckJWT)
        )
        .service(register_user_handler)
        .service(login_user_handler);

    conf.service(scope);
}