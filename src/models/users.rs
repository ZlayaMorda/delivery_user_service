use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::schema::*;
use validator::{Validate};


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

#[derive(Serialize, Insertable, Debug, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct RegisterUser {
   #[validate(length(min = 2, max = 64,
   message = "Name must be less than 64 characters and greater than 2"))]
   pub first_name: String,
   #[validate(phone)]
   pub phone_number: String,
   #[validate(email)]
   pub email: String,
   #[validate(length(min = 1, max = 100))]
   pub password: String,
}

#[derive(Debug, Queryable, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct LoginUser {
   #[validate(phone)]
   pub phone_number: String,
   #[validate(length(min = 1, max = 100))]
   pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct ResultLoginUser {
   pub user_uuid: Uuid,
   pub role: String,
   pub password: String,
}

pub struct MiddlewareUserInfo {
   pub user_uuid: Uuid,
   pub role: String,
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