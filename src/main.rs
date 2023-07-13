extern crate diesel;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;

use bb8::Pool;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

mod utils;
use utils::config::Config;

mod db;
mod models;
mod handlers;
mod middleware;
mod repository;
mod services;
mod api;

use db::db_utils::{get_pool};


pub struct AppState {
    pub db: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pub env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();
    let service_host = config.service_host.clone();
    let service_port = config.service_port.clone();

    let pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>> = get_pool(&config.db_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            }))
            .service(
                web::scope("/api")
                    .configure(api::v1::users_config)
            )
    })
    .bind((
        service_host,
        service_port
    ))?
    .run()
    .await
}