use serde::{Serialize, Deserialize};


#[Deserialize, Clone]
#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct Users {
   pub first_name: String,
   pub address: String,
   pub phone_number: String,
   pub email: String,
   pub password: String,
   pub role: String,
   pub is_blocked: bool,
   pub is_deleted: bool,
   pub created_at: NaiveDateTime,
   pub updated_at: NaiveDateTime
}

#[derive(Debug,Deserialize)]
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

#[Deserialize, Clone]
#[derive(Insertable, Debug)]
#[diesel(belongs_to(Users))]
#[diesel(table_name = couriers)]
pub struct Couriers {
   pub user_uuid: String,
   pub is_free: bool,
   pub rating: f64,
   pub created_at: NaiveDateTime,
   pub updated_at: NaiveDateTime
}