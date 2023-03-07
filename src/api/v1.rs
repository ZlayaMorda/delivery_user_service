use actix_web::web;
use crate::handlers::{
    authentication::{register_user_handler, show_users}
};
use crate::handlers::authentication::get_user_handler;

pub fn users_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(get_user_handler)
        .service(show_users);

    conf.service(scope);
}