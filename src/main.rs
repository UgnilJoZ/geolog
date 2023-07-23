use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;
use std::env;
use std::io::{Error, ErrorKind, Result};
mod database;
mod errors;
mod types;
use database::Database;
mod auth;
mod endpoints;
use endpoints::{get_points, get_track, insert_points, insert_track, list_devices, list_tracks};

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
    info!("Connected to database");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone()))
            .service(list_devices)
            .service(insert_points)
            .service(get_points)
            .service(get_track)
            .service(insert_track)
            .service(list_tracks)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
