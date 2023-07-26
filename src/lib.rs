extern crate diesel;
use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use actix_web::web::Data;

use bb8::Pool;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_actix_web::TracingLogger;

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

pub fn get_subscriber(
    config: &Config
) -> impl Subscriber + Send + Sync {

    let env_filter = EnvFilter::new(config.tracing_filter.to_string());

    let file_appender = tracing_appender::rolling::hourly(
        format!("{}/{}-logs", &config.logs_path, &config.name),
        format!("{}.log", &config.name)
    );

    //let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let formatting_layer = BunyanFormattingLayer::new(
        config.name.to_string(),
        file_appender
    );

    Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(tracing_subscriber::fmt::layer())
    .with(formatting_layer)
}

pub async fn run(mode: &str) -> Result<Server, std::io::Error> {
    let config = Config::init(mode);
    let service_host = config.service_host.clone();
    let service_port = config.service_port.clone();

    let subscriber = get_subscriber(
        &config,
    );
    set_global_default(subscriber).expect("Failed to set subscriber");

    let pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>> = get_pool(&config.db_url).await;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
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
    .run();
    Ok(server)
}