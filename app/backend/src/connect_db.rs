use std::io::Write;

use log::{error, info};
use sqlx::PgPool;
use tokio::{self, time::Duration};

pub async fn connect_to_datebase() -> Result<PgPool, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "Environment variable DATABASE_URL must be set")?;
    let max_timeout = std::env::var("DATABASE_CONNECT_TIMEOUT")
        .unwrap_or_else(|_| "600".into())
        .parse::<u64>()
        .unwrap_or(600);
    let mut backoff_timeout = 1;

    info!("Connecting to database {} ...", database_url);
    while backoff_timeout < max_timeout {
        let pool_options = sqlx::postgres::PgPoolOptions::new().max_connections(10);

        match pool_options.connect(&database_url).await {
            Ok(pool) => {
                info!("Database connection established successfully");
                return Ok(pool);
            }
            Err(e) => {
                error!("Database connection attempt failed: {:?}", e);
            }
        }
        std::io::stdout()
            .flush()
            .unwrap_or_else(|e| error!("Failed to flush stdout: {}", e));

        tokio::time::sleep(Duration::from_secs(backoff_timeout)).await;
        backoff_timeout *= 2;
    }

    Err("Database connection failed after exhausting retry attempts".into())
}
