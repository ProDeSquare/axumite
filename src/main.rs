mod app;
mod config;
mod controllers;
mod error;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    app::run().await;
}
