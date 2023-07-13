use actix_web::{HttpResponse, post, web};
use validator::Validate;

use crate::{
    AppState,
};
use crate::models::users::{LoginUser, RegisterUser};
use crate::services::users::{login_user, register_insert_user};

#[post("/auth/sign-up")]
pub async fn register_user_handler(
    body: web::Json<RegisterUser>,
    data: web::Data<AppState>,
) -> HttpResponse {

    match body.validate() {
      Ok(_) => (),
      Err(error) => return HttpResponse::BadRequest().json(
          format!("{:?}", error)
      )
    };

    register_insert_user(&body, &data).await
}

#[post("/auth/sign-in")]
pub async fn login_user_handler(
    body: web::Json<LoginUser>,
    data: web::Data<AppState>
) -> HttpResponse {
    match body.validate() {
      Ok(_) => (),
      Err(error) => return HttpResponse::BadRequest().json(
          format!("{:?}", error)
      )
    };

    login_user(&body, &data).await
}

pub async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}
