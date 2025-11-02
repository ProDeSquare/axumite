use crate::config::AppConfig;
use axum::http::{
    Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub fn build_cors(config: &AppConfig) -> CorsLayer {
    let allowed_origins_env = config.allowed_origins.clone().unwrap_or_default();
    let allowed_origins: Vec<String> = allowed_origins_env
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if allowed_origins.is_empty() || allowed_origins == ["none"] {
        info!("CORS disabled (no origins allowed)");
        return CorsLayer::new().allow_origin([]).allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ]);
    }

    if allowed_origins.contains(&"*".to_string()) {
        info!("CORS enabled for all origins (wildcard)");
        return CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
            ])
            .allow_headers(Any);
    }

    info!("CORS allowed origins: {:?}", allowed_origins);
    let origins = allowed_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect::<Vec<_>>();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
        .allow_credentials(true)
}
