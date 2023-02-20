use actix::SyncArbiter;
use dotenv::dotenv;
use std::{env,sync::Mutex};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};

mod db;
use db::db_utils::{get_pool, DbActor, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect("Failed to read .env file");

    let db_url:String = env::var("DATABASE_URL").expect("Database url must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(4, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{ db: db_addr.clone() }))
    })
    .bind((
        env::var("SERVICE_HOST").unwrap(),
        env::var("SERVICE_PORT").unwrap().parse::<u16>().unwrap()
    ))?
    .run()
    .await
}