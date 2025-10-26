use axum::{Router, routing::get, extract::State};
use crate::state::AppState;

mod health_routes;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root_handler))
        .merge(health_routes::router())
}

async fn root_handler(State(state): State<AppState>) -> String {
    let cfg = &state.config;
    format!("{} - alive and listening", cfg.name)
}
