use diesel::{insert_into, QueryDsl, QueryResult};
use bb8::{PooledConnection};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use uuid::Uuid;
use crate::models::users::{ResultLoginUser, User};
use crate::db::schema::users::dsl::*;
use crate::diesel::ExpressionMethods;
use crate::utils::errors::AuthorizationError;


pub async fn find_user<'a>(
    user_id: &Uuid,
    connection: &mut PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>
) ->  Result<User, diesel::result::Error> {

    users.find(user_id).get_result::<User>(connection).await
}

pub async fn insert_user <'a>(
    connection: &mut PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>,
    first_name_ins: &str,
    phone_number_ins: &str,
    email_ins: &str,
    password_ins: &str
) -> Result<Uuid, diesel::result::Error> {

    insert_into(users)
        .values((
            &first_name.eq(first_name_ins),
            &phone_number.eq(phone_number_ins),
            &email.eq(email_ins),
            &password.eq(password_ins)
        ))
        .returning(user_uuid)
        .get_result(connection).await
}

pub async fn find_login_user<'a>(
    connection: &mut PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>,
    phone_number_ins: &'a str
) -> Result<ResultLoginUser, AuthorizationError> {

    match users.filter(phone_number.eq(phone_number_ins)).
        select((user_uuid, role, password)).first(connection).await {
        Ok(user) => { Ok(user) },
        Err(_) => { Err(AuthorizationError::UserDoesNotExist("user does not exist".to_string())) }
    }
}
