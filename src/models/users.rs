use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::schema::*;


#[derive(Serialize, Queryable, Insertable, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct User {
   pub user_uuid: Uuid,
   pub first_name: String,
   pub address: Option<String>,
   pub phone_number: String,
   pub email: String,
   pub password: String,
   pub role: String,
   pub is_blocked: bool,
   pub is_deleted: bool,
   pub created_at: Option<NaiveDateTime>,
   pub updated_at: Option<NaiveDateTime>
}

#[derive(Serialize, Queryable, Insertable, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct RegisterUser {
   pub first_name: String,
   pub phone_number: String,
   pub email: String,
   pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
   pub phone_number: String,
   pub password: String,
}

#[derive(Deserialize, Serialize, Insertable, Clone, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = couriers)]
pub struct Couriers {
   pub user_uuid: Uuid,
   pub is_free: Option<bool>,
   pub rating: Option<f64>,
   pub created_at: Option<NaiveDateTime>,
   pub updated_at: Option<NaiveDateTime>
}