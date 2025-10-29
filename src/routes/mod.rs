use crate::{
    controllers::{error_controller::not_found_handler, root_controller::root_handler},
    state::AppState,
};
use axum::{Router, routing::get};

mod health_routes;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root_handler))
        .merge(health_routes::router())
        .fallback(not_found_handler)
}
