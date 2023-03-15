use actix_web::{App, get, HttpResponse, post, web};
use validator::Validate;

use crate::{
    AppState,
};
use crate::models::users::{LoginUser, RegisterUser, User};

use crate::repository::users::{find_user};
use crate::services::users::{login_user, register_insert_user};

#[post("/auth/register")]
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

    register_insert_user(&body, &data)
}

#[post("/auth/login")]
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

    login_user(&body, &data)
}

#[post("/auth/get")]
pub async fn get_user_handler(
    body: web::Json<User>,
    data: web::Data<AppState>,
) -> HttpResponse {

    let user = find_user(
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
