mod app;
mod config;
mod controllers;
mod db;
mod error;
mod middleware;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    app::run().await;
}
