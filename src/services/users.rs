use actix_web::{HttpResponse, web};
use crate::AppState;
use crate::models::users::{LoginUser, RegisterUser};
use crate::repository::users::{find_login_user, insert_user};
use crate::services::authentication::{check_password, generate_jwt, hashing_password};

pub fn register_insert_user<'a>(
    body: &'a web::Json<RegisterUser>,
    data: &'a web::Data<AppState>
) -> HttpResponse {

    let password_ins = hashing_password(
        &body.password,
        &data.env.salt
    );

    let inserted_result = insert_user(
        &mut data.db.get().expect("Cant get db data"),
        &body.first_name,
        &body.phone_number,
        &body.email,
        &password_ins
    );

    match inserted_result {
        Ok(user_id) => {
            let token_result= generate_jwt(&user_id, "user", &data);

            match token_result {
                Ok(token) => HttpResponse::Ok().json(
                serde_json::to_string(&token).
                    expect("Error while converting token to string")),

                Err(error) => HttpResponse::Conflict().json(
                format!("{:?}", error))
            }

        },
        Err(error) => HttpResponse::Conflict().json(
            format!("{:?}", error))
    }
}

pub fn login_user<'a>(
    body: &'a web::Json<LoginUser>,
    data: &'a web::Data<AppState>
) -> HttpResponse {

    let user = match find_login_user(
        &mut data.db.get().expect("Cant get db data"),
        & body.phone_number
    ) {
        Ok(vec_user) => {
            match vec_user.first().cloned() {
                Some(found_user) => found_user,
                None => return HttpResponse::NotFound().json(
                format!("User not found"))
            }
        },
        Err(error) => return HttpResponse::NotFound().json(
                format!("{:?}", error))
    };

    match check_password(&body.password, &data.env.salt, &user.password) {
        Ok(()) => {
            let jwt = generate_jwt(
                & user.user_uuid, & user.role, &data);

            match jwt {
                Ok(token) => HttpResponse::Ok().json(
                    serde_json::to_string(&token).
                        expect("Error while converting token to string")),
                Err(error) => return HttpResponse::Conflict().json(
                    format!("{:?}", error))
            }
        }
        Err(error) => {
            return HttpResponse::Conflict().json(
                format!("{:?}", error))
        }
    }
}