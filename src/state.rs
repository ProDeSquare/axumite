use crate::{
    config::AppConfig,
    db::{postgres::DbPool, redis::RedisPool},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub redis: RedisPool,
    pub config: Arc<AppConfig>,
}
