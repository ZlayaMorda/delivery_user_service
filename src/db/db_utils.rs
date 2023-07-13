// use actix::{Actor, SyncContext};
use bb8::{Pool};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection,
};
// pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);
//
// impl Actor for DbActor {
//     type Context = SyncContext<Self>;
// }

pub async fn get_pool(db_url: &str) -> Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
    let manager = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    bb8::Pool::builder().build(manager).await.expect("Error building a connection pool")
}

// pub fn establish_connection(database_url: &str) -> PgConnection {
//     PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url))
// }