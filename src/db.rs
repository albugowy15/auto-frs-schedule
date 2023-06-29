pub struct Connection {
    pub conn: mysql_async::Conn,
    pub pool: mysql_async::Pool,
}

impl Connection {
    pub async fn create_connection(db_url: &str) -> Result<Self, mysql_async::Error> {
        println!("Start db connection...");
        let builder = mysql_async::OptsBuilder::from_opts(mysql_async::Opts::from_url(&db_url)?);
        let pool = mysql_async::Pool::new(builder.ssl_opts(mysql_async::SslOpts::default()));
        let conn = pool.get_conn().await?;
        println!("DB Connection suceesfully establised");
        Ok(Self { conn, pool })
    }
    pub async fn close_connection(self) -> Result<(), mysql_async::Error> {
        println!("Closing DB Connection...");
        drop(self.conn);
        self.pool.disconnect().await?;
        println!("DB Connection succesfully closed");
        Ok(())
    }
}
