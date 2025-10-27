use crate::config::AppConfig;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use tracing::{error, info};

pub type DbPool = Pool;

pub async fn init_db() -> DbPool {
    let cfg = AppConfig::from_env();
    let url = cfg.postgres_url();

    info!(
        "Connecting to PostgreSQL at {}:{} / {}",
        cfg.database_host, cfg.database_port, cfg.database_name
    );

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(
        url.parse().expect("Invalid database url"),
        NoTls,
        mgr_config,
    );

    let pool = Pool::builder(mgr)
        .max_size(16)
        .build()
        .unwrap_or_else(|e| panic!("Database pool creation failed: {}", e));

    match pool.get().await {
        Ok(_) => info!("PostgreSQL connection established successfully"),
        Err(e) => {
            error!("Failed to connect to PostgreSQL: {}", e);
            panic!("Databse connection failed");
        }
    }

    pool
}
