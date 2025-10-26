use crate::{config::AppConfig, db::{DbPool, redis::RedisPool}};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub redis: Arc<RedisPool>,
    pub config: Arc<AppConfig>,
}
