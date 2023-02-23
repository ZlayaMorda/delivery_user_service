use actix::SyncArbiter;
use dotenv::dotenv;
use std::{env,sync::Mutex};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};

mod utils;
use utils::config::Config;

mod db;
use db::db_utils::{get_pool, DbActor, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();

    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&config.db_url);
    let db_addr = SyncArbiter::start(4, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{ db: db_addr.clone() }))
    })
    .bind((
        config.service_host,
        config.service_port
    ))?
    .run()
    .await
}