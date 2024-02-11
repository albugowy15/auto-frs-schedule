pub mod repository;

use std::env;

use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub struct Database;

impl Database {
    pub async fn create_connection() -> Result<MySqlPool> {
        let db_url =
            env::var("FRS_HELPER_DB_URL").with_context(|| "FRS_HELPER_DB_URL must be set")?;
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .with_context(|| "Error when create database connection")?;
        Ok(pool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_connection_no_env_var() {
        // Unset the environment variable
        env::remove_var("FRS_HELPER_DB_URL");

        // Call the function
        let result = Database::create_connection().await;

        // Check if an error is returned
        assert!(result.is_err());
    }
}
