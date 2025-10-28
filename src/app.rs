use crate::routes;
use crate::{
    apply_rate_limiter, config::AppConfig, db::postgres::init_db, db::redis::init_redis,
    state::AppState,
};
use axum::serve;
use dotenvy::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("tower_http=info".parse().unwrap())
                .add_directive("axum::rejection=trace".parse().unwrap()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let config = Arc::new(AppConfig::from_env());
    let redis = Arc::new(init_redis().await);
    let db_pool = Arc::new(init_db().await);
    let state = AppState {
        db_pool: db_pool.clone(),
        config: config.clone(),
        redis: redis.clone(),
    };

    let app = apply_rate_limiter!(routes::create_router())
        .with_state(state.clone())
        .layer(trace_layer);

    let addr = config.addr();
    info!(
        "{} listening on http://{} in {} mode",
        config.name, addr, config.env
    );

    let listener = TcpListener::bind(addr).await.unwrap();

    if let Err(err) = serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        error!("server error: {}", err);
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    println!();
    info!("graceful shutdown initiated");
}
