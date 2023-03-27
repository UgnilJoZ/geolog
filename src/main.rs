use actix_web::{middleware::Logger, web, App, HttpServer};
mod database;
mod errors;
mod types;
use database::Database;
mod auth;
mod endpoints;
use endpoints::{get_points, insert_points};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let db = Database::new().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone()))
            .service(insert_points)
            .service(get_points)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
