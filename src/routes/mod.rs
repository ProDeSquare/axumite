use crate::{controllers::error_controller::not_found_handler, state::AppState};
use axum::{Router, extract::State, routing::get};

mod health_routes;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root_handler))
        .merge(health_routes::router())
        .fallback(not_found_handler)
}

async fn root_handler(State(state): State<AppState>) -> String {
    let cfg = &state.config;
    format!("{} - alive and listening", cfg.name)
}
