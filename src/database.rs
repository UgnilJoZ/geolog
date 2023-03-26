use sqlx::{Row, Postgres, Error, Type, FromRow};
use sqlx::postgres::{PgTypeInfo, PgRow, PgPool};
use crate::types::{PointRecord, Point, Coordinates, Device};

const POINT_QUERY_BASE: &str = "SELECT id, owner, ST_X(coordinates) AS longitude,
ST_Y(coordinates) AS latitude, elevation, time, device FROM points";

const POINT_INSERTION: &str = "INSERT INTO points (owner, coordinates, elevation, time, device)
VALUES ($1, ST_SetSRID(ST_MakePoint($2, $3), 4326), $4, $5, $6)";

const DEVICE_QUERY: &str = "SELECT name, username FROM devices";

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
        sqlx::query_as(POINT_QUERY_BASE)
            .fetch_all(pool)
            .await
    }

    pub async fn insert_points(&self, points: Vec<Point>, owner: String) -> Result<(), Error> {
        let Database(pool) = self;
        for point in points.iter() {
            sqlx::query(POINT_INSERTION)
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

    pub async fn get_device(&self, token: Vec<u8>) -> Result<Device, Error> {
        let Database(pool) = self;
        sqlx::query_as(DEVICE_QUERY)
            .fetch_one(pool)
            .await
    }
}

pub struct PointFilter {
    minlon: f64,
    maxlon: f64,
    minlat: f64,
    maxlat: f64,
    device: Option<String>,
}

impl FromRow<'_, PgRow> for Coordinates {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Coordinates(row.try_get("longitude")?, row.try_get("latitude")?))
    }
}

impl Type<Postgres> for Coordinates {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("geometry(Point,4326)")
    }
}

impl FromRow<'_, PgRow> for Point {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Point {
            coordinates: Coordinates::from_row(row)?,
            elevation: row.try_get("elevation")?,
            time: row.try_get("time")?,
            device: row.try_get("device")?,
        })
    }
}
