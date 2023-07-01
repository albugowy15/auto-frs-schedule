use anyhow::{Context, Result};

pub struct Connection {
    pub conn: mysql_async::Conn,
    pub pool: mysql_async::Pool,
}

impl Connection {
    pub async fn create_connection(db_url: &str) -> Result<Self> {
        let connection_string = mysql_async::Opts::from_url(&db_url)
            .with_context(|| format!("Could not create db connection url"))?;
        let builder = mysql_async::OptsBuilder::from_opts(connection_string);
        let pool = mysql_async::Pool::new(builder.ssl_opts(mysql_async::SslOpts::default()));
        let conn = pool
            .get_conn()
            .await
            .with_context(|| format!("Error establish db connection"))?;
        Ok(Self { conn, pool })
    }
    pub async fn close_connection(self) -> Result<()> {
        drop(self.conn);
        self.pool
            .disconnect()
            .await
            .with_context(|| format!("Cannot close connection"))?;
        Ok(())
    }
}
