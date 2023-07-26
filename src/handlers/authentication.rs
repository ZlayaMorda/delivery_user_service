use actix_web::{HttpResponse, post, web};
use validator::Validate;

use crate::{
    AppState,
};
use crate::models::users::{LoginUser, RegisterUser};
use crate::services::users::{login_user, sign_up_user};

#[post("/auth/sign-up")]
#[tracing::instrument(
    name = "sign up user",
    skip(body, data),
    fields(
        phone_number = %body.phone_number
    )
)]
pub async fn sign_up_handler(
    body: web::Json<RegisterUser>,
    data: web::Data<AppState>,
) -> HttpResponse {

    match body.validate() {
      Ok(_) => (),
      Err(error) => return HttpResponse::BadRequest().json(
          format!("{:?}", error)
      )
    };

    match sign_up_user(&body, &data).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(error) => HttpResponse::Unauthorized().json(error.to_string())
    }
}

#[post("/auth/sign-in")]
#[tracing::instrument(
    name = "sign in user",
    skip(body, data),
    fields(
        phone_number = %body.phone_number
    )
)]
pub async fn sign_in_handler(
    body: web::Json<LoginUser>,
    data: web::Data<AppState>
) -> HttpResponse {

    match body.validate() {
      Ok(_) => (),
      Err(error) => return HttpResponse::BadRequest().json(
          format!("{:?}", error)
      )
    };

    match login_user(&body, &data).await {
        Ok(token) => {
            HttpResponse::Ok().json(token)
        },
        Err(error) => {
            HttpResponse::Unauthorized().json(error.to_string())
        }
    }
}

pub async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}
