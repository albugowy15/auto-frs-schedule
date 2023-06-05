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
            println!("error: {}", e);
            return;
        }
    };
    let mut conn = pool.get_conn().await.unwrap();

    // retrieve data from database
    let mut sql_data = SQLData::new();
    match sql_data.get_all_subject(&mut conn).await {
        Ok(_) => println!("success get all subject"),
        Err(e) => {
            println!("error get all subject : {}", e);
            return;
        }
    };
    match sql_data.get_all_lecture(&mut conn).await {
        Ok(_) => println!("success get all lecture"),
        Err(e) => {
            println!("error get all lecture : {}", e);
            return;
        }
    };
    match sql_data.get_all_session(&mut conn).await {
        Ok(_) => println!("success get all session"),
        Err(e) => {
            println!("error get all session : {}", e);
            return;
        }
    };
    drop(conn);
    pool.disconnect().await.unwrap();

    // println!("sql session : {:?}", sql_data.session);
    // println!("sql subject : {:?}", sql_data.subject);
    // println!("sql lecturer : {:?}", sql_data.lecturer);

    parse_excel(&sql_data.subject, &sql_data.lecturer, &sql_data.session).unwrap();

    // retrive data from excel
}
