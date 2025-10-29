use crate::state::AppState;
use axum::extract::State;

pub async fn root_handler(State(state): State<AppState>) -> String {
    let cfg = &state.config;
    format!("{} - alive and listening", cfg.name)
}
