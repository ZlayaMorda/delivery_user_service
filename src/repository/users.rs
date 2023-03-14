use diesel::{insert_into, PgConnection, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use diesel::query_builder::AsQuery;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use uuid::Uuid;
use crate::models::users::{ResultLoginUser, User};
use crate::db::schema::users::dsl::*;
use crate::diesel::ExpressionMethods;


pub fn find_user(
    user_id: &Uuid,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>
) ->  Result<User, diesel::result::Error> {
    users.find(user_id).get_result::<User>(connection)
}

pub fn insert_user(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
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
        .get_result(connection)
}

pub fn find_login_user(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    phone_number_ins: &str
) -> Result<ResultLoginUser, diesel::result::Error> {

    match users::table.filter(users::phone_number.eq(phone_number_ins)).
        load::<ResultLoginUser>(&connection) {
        Ok(user) => user,
        Err(error) => Err(error)
    }
}
