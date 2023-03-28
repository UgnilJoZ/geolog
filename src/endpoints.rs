use crate::database::{Database, PointFilter, TrackDefinition};
use crate::errors::Error;
use crate::types::{Device, Point, PointRecord, Track, TrackSpec};
use actix_web::{get, post, put, web, web::Json, HttpResponse};
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

#[get("/tracks/{name}")]
async fn get_track(
    db: web::Data<Database>,
    auth: Device,
    track_name: web::Path<String>,
) -> Result<Json<Track>, Error> {
    let track_def = db
        .get_track(track_name.into_inner())
        .await?;
    let point_filter = track_def.clone().into();
    let points = db
        .get_points(&point_filter)
        .await?;
    Ok(Json(Track {
        definition: track_def.spec,
        points,
    }))
}

#[put("/tracks/{name}")]
async fn insert_track(
    db: web::Data<Database>,
    auth: Device,
    track_name: web::Path<String>,
    track_spec: Json<TrackSpec>,
) -> Result<HttpResponse, Error> {
    db.insert_track(TrackDefinition {
        name: track_name.into_inner(),
        owner: auth.username,
        spec: track_spec.into_inner(),
    }).await?;
    Ok(HttpResponse::Created().into())
}
