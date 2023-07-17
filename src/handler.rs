use crate::{
    model::WeatherModel,
    schemas::{CreateWeatherSchema, FilterOptions, UpdateWeatherSchema},
    AppState,
};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/weather")]
pub async fn weather_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        WeatherModel,
        "SELECT * FROM weather ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happend while fetching all weather items";
        return HttpResponse::InternalServerError()
        .json(json!({"status": "error","message": message}));
    }

    let weather = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "sucess",
        "results": weather.len(),
        "weather": weather
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/weather")]
async fn create_weather_handler(
    body: web::Json<CreateWeatherSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        WeatherModel,
        "INSERT INTO weather (city,temperature, description, humidity, wind_speed) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        body.city.to_string(),
        body.temperature.to_string(),
        body.description.as_ref().map(|s| s.as_str()).unwrap_or_default(),
        body.humidity.as_ref().map(|s| s.as_str()).unwrap_or_default(),
        body.wind_speed.as_ref().map(|s| s.as_str()).unwrap_or_default()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(weather) => {
            let weather_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "weather": weather
            })});

            return HttpResponse::Ok().json(weather_response);
        }
        Err(e) => {
            HttpResponse::InternalServerError()
            .json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            }))
        }
    }
}

#[patch("/weather/{id}")]
async fn edit_weather_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateWeatherSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let weather_id = path.into_inner();
    let query_result = sqlx::query_as!(WeatherModel, "SELECT * FROM weather WHERE id = $1", weather_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Weather with ID: {} not found", weather_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}))
    }

    let now = Utc::now();
    let weather = query_result.unwrap();

    let query_result = sqlx::query_as!(
        WeatherModel,
        "UPDATE weather SET city = $1, temperature = $2, description = $3, humidity = $4, wind_speed = $5, updated_at = $6 WHERE id = $7 RETURNING *",
        body.city.to_owned().unwrap_or(weather.city.clone()),
        body.temperature.to_owned().unwrap_or(weather.temperature.clone()),
        body.description.to_owned().unwrap_or_else(|| weather.description.to_owned().unwrap_or_default()),
        body.humidity.to_owned().unwrap_or_else(|| weather.humidity.to_owned().unwrap_or_default()),
        body.wind_speed.to_owned().unwrap_or_else(|| weather.wind_speed.to_owned().unwrap_or_default()),
        now,
        weather_id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(weather) => {
            let weather_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "weather": weather
            })});

            return HttpResponse::Ok().json(weather_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/weather/{id}")]
async fn delete_weather_handler(
    path: web:: Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let weather_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM weather WHERE id = $1", weather_id)
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let message = format!("Weather with ID: {} not found", weather_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api")
    .service(weather_list_handler)
    .service(create_weather_handler)
    .service(edit_weather_handler)
    .service(delete_weather_handler);

    conf.service(scope);
}