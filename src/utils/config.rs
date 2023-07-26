use dotenv::{from_filename};
use std::{env};

#[derive(Debug,Clone)]
pub struct Config {
    pub service_host: String,
    pub service_port: u16,
    pub db_url: String,
    pub secret: String,
    pub expires_in_days: u16,
    pub expires_in_minutes: u16,
    pub salt: String,
    pub logs_path: String,
    pub name: String,
    pub tracing_filter: String,
}

impl Config {

    pub fn init(mode: &str) -> Config {
        match mode {
            "dev" => { from_filename(".env.dev").expect("Failed to read .env.dev.dev file"); }
            "prod" => { from_filename(".env.prod").expect("Failed to read .env.dev.prod file"); }
            _ => { from_filename(".env.test").expect("Failed to read .env.dev.test file"); }
        }
        let service_host= env::var("SERVICE_HOST")
            .expect("Service host must be set");
        let service_port = env::var("SERVICE_PORT")
            .expect("Service port must be set")
            .parse::<u16>().expect("Service port must be u16");
        let db_url = env::var("DATABASE_URL")
            .expect("Database url must be set");
        let secret = env::var("SECRET")
            .expect("Secret key must be set");
        let expires_in_days = env::var("EXPIRES_IN_DAYS")
            .expect("JWT expires in days must be set")
            .parse::<u16>().expect("Expires in days must be u16");
        let expires_in_minutes = env::var("EXPIRES_IN_MINUTES")
            .expect("JWT expires in minutes must be set")
            .parse::<u16>().expect("Expires in minutes must be u16");
        let salt = env::var("SALT")
            .expect("Solt must be set");
        let logs_path = env::var("LOGS_PATH")
            .expect("Logs path must be set");
        let name = env::var("NAME")
            .expect("App name must be set");
        let tracing_filter = env::var("TRACING_FILTER")
            .expect("Tracing filter must be set");

        Config {
            service_host,
            service_port,
            db_url,
            secret,
            expires_in_days,
            expires_in_minutes,
            salt,
            logs_path,
            name,
            tracing_filter,
        }
    }
}