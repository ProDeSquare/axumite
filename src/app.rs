use axum::serve;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{Level, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::env;

use crate::routes;

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

    let app = routes::create_router().layer(trace_layer);

    let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("{}:{}", host, port);
    info!("prodesquare_api listening on http://{}", addr);

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
    info!("graceful shutdown initiated");
}
