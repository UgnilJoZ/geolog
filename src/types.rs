use std::num::ParseIntError;
use serde::{Serialize, Deserialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

/// WGS84 lon/lat tuple
#[derive(Serialize, Deserialize)]
pub struct Coordinates(pub f64, pub f64);

impl Coordinates {
    pub fn longitude(&self) -> f64 {
        self.0
    }

    pub fn latitude(&self) -> f64 {
        self.1
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

#[derive(Serialize, sqlx::FromRow)]
pub struct PointRecord {
    pub id: PointId,
    pub owner: String,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub body: Point,
}

#[derive(Deserialize)]
#[derive(sqlx::Type)]
#[sqlx(transparent)]
pub struct DeviceToken(Vec<u8>);

impl DeviceToken {
    pub fn from_base64(base64_string: impl AsRef<[u8]>) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode(base64_string)
    }
}

#[derive(Deserialize)]
#[derive(FromRow)]
pub struct Device {
    #[serde(rename = "device")]
    pub name: String,
    pub username: String,
}
