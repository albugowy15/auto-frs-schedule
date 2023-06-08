use auto_frs_schedule::{
    db::{drop_old_data, insert_data, SQLData},
    excel::Excel,
};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // start db connection
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("\nStart db connection");
    let builder =
        mysql_async::OptsBuilder::from_opts(mysql_async::Opts::from_url(&db_url).unwrap());
    let pool = mysql_async::Pool::new(builder.ssl_opts(mysql_async::SslOpts::default()));
    let mut conn = match pool.get_conn().await {
        Ok(conn) => {
            println!("Success establish db connection");
            conn
        }
        Err(e) => {
            panic!("Error establish db connection : {}", e);
        }
    };

    // retrieve data from database
    println!("\nStart retrieve data from database");
    let mut sql_data = SQLData::new();
    match sql_data.get_all_subject(&mut conn).await {
        Ok(_) => println!("Success get all subject"),
        Err(e) => {
            panic!("Error get all subject : {}", e);
        }
    };
    match sql_data.get_all_lecture(&mut conn).await {
        Ok(_) => println!("Success get all lecture"),
        Err(e) => {
            panic!("Error get all lecture : {}", e);
        }
    };
    match sql_data.get_all_session(&mut conn).await {
        Ok(_) => println!("Success get all session"),
        Err(e) => {
            panic!("Error get all session : {}", e);
        }
    };

    // parse excel
    println!("\nStart parse excel");
    let path = format!(
        "{}/assets/Jadwal Kuliah Genap 22-23 T.Informatika ITS.xlsx",
        env!("CARGO_MANIFEST_DIR")
    );
    let excel = match Excel::new(&path, &"Jadwal Kuliah".to_string()) {
        Ok(excel) => {
            println!("Success open excel");
            excel
        }
        Err(e) => {
            panic!("Error open excel : {}", e);
        }
    };
    let list_class =
        match excel.parse_excel(&sql_data.subject, &sql_data.lecturer, &sql_data.session) {
            Ok(list_class) => {
                println!("Succesfully parse {} classes", list_class.len());
                list_class
            }
            Err(e) => {
                panic!("Error parse excel : {}", e);
            }
        };

    println!("\nStart insert classes to database");
    // insert data to database
    match drop_old_data(&mut conn).await {
        Ok(_) => println!("Successfully delete old classes"),
        Err(e) => {
            panic!("Error delete old classes : {}", e);
        }
    };
    match insert_data(&mut conn, list_class).await {
        Ok(_) => println!("Successfully insert classes"),
        Err(e) => {
            panic!("Error insert classes : {}", e);
        }
    };

    drop(conn);
    pool.disconnect().await.unwrap();
}
