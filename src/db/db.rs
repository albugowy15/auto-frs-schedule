use mysql_async::{
    prelude::{Query, WithParams},
    Conn, Error,
};

struct Subject {
    id: String,
    name: Option<String>,
}

pub async fn start_connection(database_url: String) -> Result<Conn, Error> {
    let builder =
        mysql_async::OptsBuilder::from_opts(mysql_async::Opts::from_url(&database_url).unwrap());

    let pool = mysql_async::Pool::new(builder.ssl_opts(mysql_async::SslOpts::default()));
    return pool.get_conn().await;
}

pub async fn get_all_subject_id(conn: &mut Conn) -> Result<(), Error> {
    let loaded_subject = "SELECT id, name FROM Matkul"
        .with(())
        .map(conn, |(id, name)| Subject { id, name })
        .await?;

    for subject in loaded_subject {
        match subject.name {
            Some(name) => println!("Subject: {}", name),
            None => println!("Subject: {}", subject.id),
        }
    }

    Ok(())
}
