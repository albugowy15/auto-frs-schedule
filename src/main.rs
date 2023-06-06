use auto_frs_schedule::{
    db::{drop_class_table, insert_data, SQLData},
    excel::parse_excel,
};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // start db connection
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}...", db_url);
    let builder =
        mysql_async::OptsBuilder::from_opts(mysql_async::Opts::from_url(&db_url).unwrap());

    let pool = mysql_async::Pool::new(builder.ssl_opts(mysql_async::SslOpts::default()));
    let mut conn = pool.get_conn().await.unwrap();

    // retrieve data from database
    let mut sql_data = SQLData::new();
    match sql_data.get_all_subject(&mut conn).await {
        Ok(_) => println!("success get all subject"),
        Err(e) => {
            panic!("error get all subject : {}", e);
        }
    };
    match sql_data.get_all_lecture(&mut conn).await {
        Ok(_) => println!("success get all lecture"),
        Err(e) => {
            panic!("error get all lecture : {}", e);
        }
    };
    match sql_data.get_all_session(&mut conn).await {
        Ok(_) => println!("success get all session"),
        Err(e) => {
            panic!("error get all session : {}", e);
        }
    };

    // parse excel
    let list_class = parse_excel(&sql_data.subject, &sql_data.lecturer, &sql_data.session).unwrap();

    // insert data to database
    match drop_class_table(&mut conn).await {
        Ok(_) => println!("successfully drop table"),
        Err(e) => {
            panic!("error drop table : {}", e);
        }
    };
    match insert_data(&mut conn, list_class).await {
        Ok(_) => println!("successfully insert data"),
        Err(e) => {
            panic!("error insert data : {}", e);
        }
    };

    drop(conn);
    pool.disconnect().await.unwrap();
}
