use axum::{extract::State, http::StatusCode, Json};
use crate::{error::AppError, models::health_model::Health, state::AppState};

pub async fn check_health(State(state): State<AppState>) -> Result<(StatusCode, Json<Health>), AppError> {
    let _ = state.db_pool;
    let _ = state.config;
    let _ = state.redis;
    let data = Health::ok();
    Ok((StatusCode::OK, Json(data)))
}
