use axum::{Router, routing::get};
use crate::controllers::health_controller;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health_controller::check_health))
}
