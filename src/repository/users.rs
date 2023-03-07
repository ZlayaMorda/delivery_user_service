use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};

use uuid::Uuid;
use crate::models::users::{User};
use crate::db::schema::users::dsl::*;

pub fn user_phone_exist(
    user_id: &Uuid,
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>
) ->  Result<User, diesel::result::Error> {
    users.find(user_id).get_result::<User>(connection)
}
