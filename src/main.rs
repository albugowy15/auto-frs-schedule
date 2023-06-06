use auto_frs_schedule::{
    db::{start_connection, SQLData},
    excel::parse_excel,
};
use dotenv::dotenv;

use std::env;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    print!("Connecting to {}...", db_url);
    let pool = match start_connection(db_url).await {
        Ok(conn) => {
            println!("success");
            conn
        }
        Err(e) => {
            panic!("error: {}", e);
        }
    };
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

    let list_class = parse_excel(&sql_data.subject, &sql_data.lecturer, &sql_data.session).unwrap();
    match sql_data.drop_class_table(&mut conn).await {
        Ok(_) => println!("successfully drop table"),
        Err(e) => {
            panic!("error drop table : {}", e);
        }
    };
    match sql_data.insert_data(&mut conn, list_class).await {
        Ok(_) => println!("successfully insert data"),
        Err(e) => {
            panic!("error insert data : {}", e);
        }
    };

    drop(conn);
    pool.disconnect().await.unwrap();
}
