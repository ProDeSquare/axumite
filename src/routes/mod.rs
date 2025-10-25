use axum::{Router, routing::get};

mod health_routes;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .merge(health_routes::router())
}

async fn root_handler() -> &'static str {
    "prodesquare_api - alive and listening"
}
