pub struct Connection {
    pub conn: mysql_async::Conn,
    pub pool: mysql_async::Pool,
}

impl Connection {
    pub async fn create_connection(db_url: &str) -> Result<Self, mysql_async::Error> {
        let builder = mysql_async::OptsBuilder::from_opts(mysql_async::Opts::from_url(&db_url)?);
        let pool = mysql_async::Pool::new(builder.ssl_opts(mysql_async::SslOpts::default()));
        let conn = pool.get_conn().await?;
        Ok(Self { conn, pool })
    }
    pub async fn close_connection(self) -> Result<(), mysql_async::Error> {
        drop(self.conn);
        self.pool.disconnect().await?;
        Ok(())
    }
}
