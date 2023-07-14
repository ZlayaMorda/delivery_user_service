use actix_web::{web};
use crate::AppState;
use crate::models::users::{LoginUser, RegisterUser};
use crate::repository::users::{find_login_user, insert_user};
use crate::services::authentication::{check_password, generate_jwt, hashing_password};
use crate::utils::errors::AuthorizationError;

pub async fn sign_up_user<'a>(
    body: &'a web::Json<RegisterUser>,
    data: &'a web::Data<AppState>
) -> Result<String, AuthorizationError> {

    let password_ins = hashing_password(
        &body.password,
        &data.env.salt
    );

    let user_id = insert_user(
        &mut data.db.get().await.expect("Connection must be initialized in the main"),
        &body.first_name,
        &body.phone_number,
        &body.email,
        &password_ins
    ).await?;

    Ok(generate_jwt(&user_id, "user", &data)?)
}

pub async fn login_user<'a>(
    body: &'a web::Json<LoginUser>,
    data: &'a web::Data<AppState>
) -> Result<String, AuthorizationError> {

    let user = find_login_user(
        &mut data.db.get().await.expect("Connection must be initialized in the main"),
        & body.phone_number
    ).await?;

    check_password(&body.password, &data.env.salt, &user.password)?;

    Ok(generate_jwt(& user.user_uuid, & user.role, &data)?)
}