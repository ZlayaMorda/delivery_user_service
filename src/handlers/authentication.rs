use actix_web::{
    get, HttpResponse, post, web
};

use crate::{
    AppState,
    models::{
        users::{User},
    },
};
use crate::models::users::RegisterUser;

use crate::repository::users::user_phone_exist;

#[post("/auth/register")]
pub async fn register_user_handler(
    body: web::Json<RegisterUser>,
    data: web::Data<AppState>,
) {

}

#[post("/auth/get")]
pub async fn get_user_handler(
    body: web::Json<User>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let user = user_phone_exist(
        &body.user_uuid,
        &mut data.db.get().unwrap()
    );
    match user {
        Ok(result) => HttpResponse::Ok().json(
            serde_json::to_string(&result).unwrap()),
        Err(error) => HttpResponse::InternalServerError().json(
            format!("{:?}", error))
    }
}

#[get("/show")]
pub async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}
