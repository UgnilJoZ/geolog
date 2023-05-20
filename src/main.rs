use actix_web::{middleware::Logger, web, App, HttpServer};
use std::env;
use std::io::{Error, ErrorKind, Result};
mod database;
mod errors;
mod types;
use database::Database;
mod auth;
mod endpoints;
use endpoints::{get_points, get_track, insert_points, insert_track};

#[actix_web::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let pg_url = env::var("POSTGRES_URL").map_err(|e| {
        Error::new(
            ErrorKind::NotFound,
            "The environment variable POSTGRES_URL was expected, but not supplied.",
        )
    })?;

    let db = Database::new(&pg_url).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone()))
            .service(insert_points)
            .service(get_points)
            .service(get_track)
            .service(insert_track)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
