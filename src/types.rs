use serde::{Serialize, Deserialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::{FromRow, postgres::PgRow, Row, Type, Postgres, postgres::PgTypeInfo};

/// WGS84 lon/lat tuple
#[derive(Serialize, Deserialize)]
pub struct Coordinates(f64, f64);

impl Coordinates {
    pub fn longitude(&self) -> f64 {
        self.0
    }

    pub fn latitude(&self) -> f64 {
        self.1
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

#[derive(Deserialize, Serialize)]
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct PointId(i64);

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub coordinates: Coordinates,
    pub elevation: f64,
    #[serde(with = "time::serde::iso8601")]
    pub time: OffsetDateTime,
    pub device: String,
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

#[derive(Serialize, sqlx::FromRow)]
pub struct PointRecord {
    pub id: PointId,
    pub owner: String,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub body: Point,
}
