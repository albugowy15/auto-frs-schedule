use anyhow::Result;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub struct Connection;

impl Connection {
    pub async fn create_connection(db_url: &str) -> Result<Pool<MySql>> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        Ok(pool)
    }
}
