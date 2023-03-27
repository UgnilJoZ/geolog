use sqlx::{Row, Postgres, Error, Type, FromRow, QueryBuilder};
use sqlx::postgres::{PgTypeInfo, PgRow, PgPool};
use crate::types::{PointRecord, Point, Coordinates, Device};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

const POINT_QUERY_BASE: &str = "SELECT id, owner, ST_X(coordinates) AS longitude,
ST_Y(coordinates) AS latitude, elevation, time, device FROM points";

const POINT_INSERTION: &str = "INSERT INTO points (owner, coordinates, elevation, time, device)
VALUES ($1, ST_SetSRID(ST_MakePoint($2, $3), 4326), $4, $5, $6)";

const DEVICE_QUERY: &str = "SELECT name, username FROM devices WHERE token = $1";

const EPSG_PROJECTION: i32 = 4326;

#[derive(Clone)]
pub struct Database(PgPool);

impl Database {
    pub async fn new() -> Result<Database, Error> {
        PgPool::connect("postgresql://localhost/geolog")
            .await
            .map(Database)
    }

    pub async fn get_points(&self, filter: &PointFilter) -> Result<Vec<PointRecord>, Error> {
        let Database(pool) = self;
        filter.pg_selection().build_query_as()
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
            .bind(&token)
            .fetch_one(pool)
            .await
    }
}

#[serde_as]
#[derive(Deserialize)]
pub struct BoundingBox {
    #[serde_as(as = "DisplayFromStr")]
    pub minlon: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub maxlon: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub minlat: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub maxlat: f64,
}

#[derive(Deserialize)]
pub struct PointFilter {
    pub limit: Option<i64>,
    #[serde(flatten)]
    pub bbox: Option<BoundingBox>,
    pub device: Option<String>,
    #[serde(skip_deserializing)]
    pub user: String,
}

impl PointFilter {
    fn pg_selection(&self) -> QueryBuilder<Postgres> {
        let mut query = QueryBuilder::new(POINT_QUERY_BASE);
        query.push(" WHERE owner = ");
        query.push_bind(&self.user);
        if let Some(devicename) = &self.device {
            query.push(" AND device = ");
            query.push_bind(devicename);
        }
        if let Some(bbox) = &self.bbox {
            query.push(" AND ST_Intersects(coordinates, ST_MakeEnvelope ( ");
            let mut envelope = query.separated(", ");
            envelope.push_bind(bbox.minlon);
            envelope.push_bind(bbox.minlat);
            envelope.push_bind(bbox.maxlon);
            envelope.push_bind(bbox.maxlat);
            envelope.push_bind(EPSG_PROJECTION);
            query.push(" )::geography('POLYGON') )");
        }
        if let Some(limit) = self.limit {
            query.push(" LIMIT ");
            query.push_bind(limit);
        }
        query
    }
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
