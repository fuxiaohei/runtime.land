use anyhow::Result;
use envconfig::Envconfig;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::debug;

#[derive(Envconfig, Debug, Serialize, Deserialize)]
pub struct DbConfig {
    #[envconfig(from = "MONI_DB_HOST", default = "localhost")]
    pub host: String,
    #[envconfig(from = "MONI_DB_PORT", default = "3306")]
    pub port: u16,
    #[envconfig(from = "MONI_DB_USER", default = "root")]
    pub user: String,
    #[envconfig(from = "MONI_DB_PASSWORD", default = "")]
    pub password: String,
    #[envconfig(from = "MONI_DB_NAME", default = "moni-serverless")]
    pub database: String,
    #[envconfig(from = "MONI_DB_POOL_SIZE", default = "10")]
    pub pool_size: u32,
}

impl DbConfig {
    pub fn url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "".to_string(),
            database: "moss-serverless".to_string(),
            pool_size: 10,
        }
    }
}

/// DB connection pool
pub static DB: OnceCell<DatabaseConnection> = OnceCell::new();

/// init initializes database connection pool
pub async fn init() -> Result<()> {
    let cfg = DbConfig::init_from_env().unwrap();
    let url = cfg.url();
    debug!("connect to database: {url}");

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(cfg.pool_size)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;
    DB.set(db).unwrap();
    Ok(())
}
