use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub env: String,
    pub host: String,
    pub port: u16,
    pub database_host: String,
    pub database_port: u16,
    pub database_user: String,
    pub database_pass: String,
    pub database_name: String,
    pub redis_url: String,
    pub allowed_origins: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            name: env::var("APP_NAME").unwrap_or_else(|_| "prodesquare_api".into()),
            env: env::var("APP_ENV").unwrap_or_else(|_| "production".into()),
            host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env::var("APP_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .unwrap(),
            database_host: env::var("DATABASE_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            database_port: env::var("DATABASE_PORT")
                .unwrap_or_else(|_| "5432".into())
                .parse()
                .unwrap(),
            database_user: env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".into()),
            database_pass: env::var("DATABASE_PASS").unwrap_or_default(),
            database_name: env::var("DATABASE_NAME").unwrap_or_else(|_| "prodesquare".into()),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL not set"),
            allowed_origins: env::var("ALLOWED_ORIGINS").ok(),
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database_user,
            self.database_pass,
            self.database_host,
            self.database_port,
            self.database_name
        )
    }
}
