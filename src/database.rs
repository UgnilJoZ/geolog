use sqlx::postgres::PgPool;
use sqlx::Error;
use crate::types::{PointRecord, Point};

const point_query_base: &str = "SELECT id, owner, ST_X(coordinates) AS longitude,
ST_Y(coordinates) AS latitude, elevation, time, device FROM points";

const point_insertion: &str = "INSERT INTO points (owner, coordinates, elevation, time, device)
VALUES ($1, ST_SetSRID(ST_MakePoint($2, $3), 4326), $4, $5, $6)";

#[derive(Clone)]
pub struct Database(PgPool);

impl Database {
    pub async fn new() -> Result<Database, Error> {
        PgPool::connect("postgresql://localhost/geolog")
            .await
            .map(Database)
    }

    pub async fn get_points(&self) -> Result<Vec<PointRecord>, Error> {
        let Database(pool) = self;
        sqlx::query_as(point_query_base)
            .fetch_all(pool)
            .await
    }

    pub async fn insert_points(&self, points: Vec<Point>, owner: String) -> Result<(), Error> {
        let Database(pool) = self;
        for point in points.iter() {
            sqlx::query(point_insertion)
                .bind(&owner)
                .bind(point.coordinates.longitude())
                .bind(point.coordinates.latitude())
                .bind(point.elevation)
                .bind(point.time)
                .bind(&point.device)
                .execute(pool)
                .await?;
        }
        Ok(())
    }
}
