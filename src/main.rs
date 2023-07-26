use user_service::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    run("dev").await?.await
}