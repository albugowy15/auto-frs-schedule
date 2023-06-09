use auto_frs_schedule::{
    db::{drop_old_data, insert_data, start_db_connection, SQLData},
    excel::Excel,
    Class,
};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ExcelToDB {
        #[arg(short, long, value_name = "Path to excel file")]
        file: Option<PathBuf>,
        #[arg(short, long, value_name = "Sheet name")]
        sheet: String,
    },
    ParseExcel {
        #[arg(short, long, value_name = "Path to excel file")]
        file: Option<PathBuf>,
        #[arg(short, long, value_name = "Sheet name")]
        sheet: String,
        #[arg(short, long, value_name = "Output directory")]
        outdir: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ExcelToDB { file, sheet }) => {
            // start db connection
            let path_to_excel = match file {
                Some(path) => path.to_str().unwrap().to_string(),
                None => {
                    panic!("Error file path not found");
                }
            };
            let (mut conn, pool) = match start_db_connection(&db_url).await {
                Ok((conn, pool)) => (conn, pool),
                Err(e) => {
                    panic!("Error start db connection : {}", e);
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
            let list_class = parse_excel(&path_to_excel, &sheet, &sql_data);
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
        Some(Commands::ParseExcel {
            file,
            sheet,
            outdir,
        }) => {
            // start db connection
            println!("\nStart db connection");
            let path_to_excel = match file {
                Some(path) => path.to_str().unwrap().to_string(),
                None => {
                    panic!("Error file path not found");
                }
            };
            let (mut conn, pool) = match start_db_connection(&db_url).await {
                Ok((conn, pool)) => (conn, pool),
                Err(e) => {
                    panic!("Error start db connection : {}", e);
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
            drop(conn);
            pool.disconnect().await.unwrap();
            // parse excel
            let list_class = parse_excel(&path_to_excel, &sheet, &sql_data);
            let path_output = match outdir {
                Some(path) => path.to_str().unwrap().to_string(),
                None => {
                    panic!("Error output path not found")
                }
            };
            write_output(&path_output, &list_class).await;
        }
        None => {
            println!("no command");
        }
    }
}

fn parse_excel(path_to_excel: &String, sheet: &String, sql_data: &SQLData) -> Vec<Class> {
    // parse excel
    println!("\nStart parse excel");
    let excel = match Excel::new(&path_to_excel, &sheet) {
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
    list_class
}

async fn write_output(path_output: &String, list_class: &Vec<Class>) {
    let mut outfile = match File::create(format!("{}/out.txt", path_output)).await {
        Ok(file) => {
            println!("Success create file");
            file
        }
        Err(e) => {
            panic!("Error create file : {}", e);
        }
    };
    for class in list_class {
        let line = format!("{:?}", class);
        outfile.write_all(line.as_bytes()).await.unwrap();
        outfile.write_all(b"\n").await.unwrap();
    }
}
