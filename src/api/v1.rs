use actix_web::web;
use crate::handlers::{
    authentication::{register_user_handler, show_users, get_user_handler, login_user_handler}
};


pub fn users_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(get_user_handler)
        .service(show_users)
        .service(register_user_handler)
        .service(login_user_handler);

    conf.service(scope);
}