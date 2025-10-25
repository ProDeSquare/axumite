use axum::{http::StatusCode, Json};
use crate::{models::health_model::Health, error::AppError};

pub async fn check_health() -> Result<(StatusCode, Json<Health>), AppError> {
    let data = Health::ok();
    Ok((StatusCode::OK, Json(data)))
}
