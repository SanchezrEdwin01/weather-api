use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateWeatherSchema {
    pub city: String,
    pub temperature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub humidity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateWeatherSchema {
    pub city: Option<String>,
    pub temperature: Option<String>,
    pub description: Option<String>,
    pub humidity: Option<String>,
    pub wind_speed: Option<String>,
    pub updated_at: Option<String>,
}