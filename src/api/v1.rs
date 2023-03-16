use actix_web::web;
use crate::handlers::{
    authentication::{register_user_handler, show_users, login_user_handler}
};
use crate::middleware::jwt_middleware::FactoryCheckJWT;


pub fn users_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(
            web::resource("/show").route(web::get().to(show_users)).
                wrap(FactoryCheckJWT)
        )
        .service(register_user_handler)
        .service(login_user_handler);

    conf.service(scope);
}