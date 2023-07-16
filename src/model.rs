use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::DateTime;

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct WeatherModel {
    pub id: Uuid,
    pub city: String,
    pub temperature: String,
    pub description: Option<String>,
    pub humidity: Option<String>,
    pub wind_speed: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<chrono::Utc>>,
}
