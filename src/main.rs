mod app;
mod routes;
mod controllers;
mod models;

#[tokio::main]
async fn main() {
    app::run().await;
}
