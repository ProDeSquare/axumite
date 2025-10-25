use axum::{http::StatusCode, Json};
use crate::models::health_model::Health;

pub async fn check_health() -> (StatusCode, Json<Health>) {
    let data = Health::ok();
    (StatusCode::OK, Json(data))
}
