use crate::config::AppConfig;
use redis::{Client, aio::ConnectionManager};
use tracing::info;

pub type RedisPool = ConnectionManager;

pub async fn init_redis() -> RedisPool {
    let cfg = AppConfig::from_env();
    info!("Connecting to redis at: {}", cfg.redis_url);

    let client = Client::open(cfg.redis_url).expect("Failed to create Redis client");

    let connection = client
        .get_connection_manager()
        .await
        .expect("Failed to connect to Redis");

    info!("Redis connection established successfully");
    connection
}
