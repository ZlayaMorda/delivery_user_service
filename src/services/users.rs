use actix_web::{HttpResponse, web};
use serde_json::json;
use crate::AppState;
use crate::models::users::RegisterUser;
use crate::repository::users::insert_user;
use crate::services::authentication::{generate_jwt, hashing_password};

pub fn register_insert_user<'a>(
    body: &'a web::Json<RegisterUser>,
    data: &'a web::Data<AppState>
) -> HttpResponse {

    let hash_result = hashing_password(
        &body.password,
        &data.env.salt
    );

    let password_ins = match hash_result {
        Ok(result) => result.to_string(),
        Err(error) => return HttpResponse::Conflict().json(
            format!("{:?}", error))
    };

    let inserted_result = insert_user(
        &mut data.db.get().expect("Cant get db data"),
        &body.first_name,
        &body.phone_number,
        &body.email,
        &password_ins
    );

    match inserted_result {
        Ok(user_id) => {
            let token_result= generate_jwt(&user_id, &data);

            match token_result {
                Ok(token) => HttpResponse::Ok().json(
                serde_json::to_string(&token).unwrap()),

                Err(error) => HttpResponse::Conflict().json(
                format!("{:?}", error))
            }

        },
        Err(error) => HttpResponse::Conflict().json(
            format!("{:?}", error))
    }
}