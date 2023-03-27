extern crate serde_with;
use actix_web::{
    get,
    post,
    web,
    web::Json, App, HttpResponse, HttpServer,
    middleware::Logger};
use log::error;
mod types;
use types::{Point, PointRecord, Device};
mod errors;
use errors::Error;
mod database;
use database::{Database, PointFilter};
mod auth;

#[post("/points")]
async fn insert_points(
    Json(points): Json<Vec<Point>>,
    db: web::Data<Database>,
    auth: Device,
) -> HttpResponse {
    match db.insert_points(points, "joz".to_string()).await {
        Ok(()) => HttpResponse::Created().into(),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().into()
        }
    }
}

#[get("/points")]
async fn get_points(
    db: web::Data<Database>,
    auth: Device,
    params: web::Query<PointFilter>,
) -> Result<Json<Vec<PointRecord>>, Error> {
    let web::Query(mut params) = params;
    params.user = auth.username;
    Ok(Json(db.get_points(&params).await?))
}

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
