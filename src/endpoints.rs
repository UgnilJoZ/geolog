use crate::database::{Database, PointFilter};
use crate::errors::Error;
use crate::types::{Device, Point, PointRecord};
use actix_web::{get, post, web, web::Json, HttpResponse};
use log::error;

#[post("/points")]
async fn insert_points(
    Json(points): Json<Vec<Point>>,
    db: web::Data<Database>,
    auth: Device,
) -> HttpResponse {
    match db.insert_points(points, auth.username).await {
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
