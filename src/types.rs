use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Serialize, sqlx::Type)]
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

#[derive(Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct DeviceToken(Vec<u8>);

impl DeviceToken {
    pub fn from_base64(base64_string: impl AsRef<[u8]>) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode(base64_string)
    }
}

#[derive(Deserialize, FromRow)]
pub struct Device {
    #[serde(rename = "device")]
    pub name: String,
    pub username: String,
}

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct TrackSpec {
    pub device: String,
    #[serde(with = "time::serde::iso8601")]
    pub min_date: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub max_date: OffsetDateTime,
}

#[derive(Serialize)]
pub struct Track {
    pub definition: TrackSpec,
    pub points: Vec<PointRecord>,
}

