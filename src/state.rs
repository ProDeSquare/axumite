use crate::{config::AppConfig, db::DbPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub config: Arc<AppConfig>,
}
