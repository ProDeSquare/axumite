mod app;
mod routes;
mod controllers;
mod models;
mod error;

#[tokio::main]
async fn main() {
    app::run().await;
}
