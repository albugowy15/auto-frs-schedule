use auto_frs_schedule::{
    db::Connection,
    excel::Excel,
    repo::{Class, ClassRepository},
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let cli = Cli::parse();
    let mut db = Connection::create_connection(&db_url).await?;

    match &cli.command {
        Some(Commands::ExcelToDB { file, sheet }) => {
            let path_to_excel = match file {
                Some(path) => path.to_str().unwrap().to_string(),
                None => {
                    panic!("Error file path not found");
                }
            };
            // retrieve data from database
            println!("\nStart retrieve data from database");
            let mut class_repo = ClassRepository::new();
            class_repo.get_all_subject(&mut db.conn).await?;
            class_repo.get_all_lecture(&mut db.conn).await?;
            class_repo.get_all_session(&mut db.conn).await?;
            // parse excel
            let list_class = parse_excel(&path_to_excel, &sheet, &class_repo);
            println!("\nStart insert classes to database");
            // insert data to database
            class_repo.insert_data(&mut db.conn, list_class).await?;
        }
        Some(Commands::ParseExcel {
            file,
            sheet,
            outdir,
        }) => {
            let path_to_excel = match file {
                Some(path) => path.to_str().unwrap().to_string(),
                None => {
                    panic!("Error file path not found");
                }
            };
            // retrieve data from database
            println!("\nStart retrieve data from database");
            let mut class_repo = ClassRepository::new();
            class_repo.get_all_subject(&mut db.conn).await?;
            class_repo.get_all_lecture(&mut db.conn).await?;
            class_repo.get_all_session(&mut db.conn).await?;
            // parse excel
            let list_class = parse_excel(&path_to_excel, &sheet, &class_repo);
            let path_output = match outdir {
                Some(path) => path.to_str().unwrap().to_string(),
                None => {
                    panic!("Error output path not found")
                }
            };
            write_output(&path_output, &list_class).await?;
        }
        None => {
            println!("no command");
        }
    }
    db.close_connection().await?;
    Ok(())
}

fn parse_excel(path_to_excel: &String, sheet: &String, sql_data: &ClassRepository) -> Vec<Class> {
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
        match excel.parse_excel(&sql_data.subjects, &sql_data.lecturers, &sql_data.sessions) {
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

#[allow(deprecated)]
async fn write_output(
    path_output: &String,
    list_class: &Vec<Class>,
) -> Result<(), tokio::io::Error> {
    let mut outfile = File::create(format!("{}/out.sql", path_output))
        .await
        .expect("Error create directory");
    for class in list_class {
        let id_class = cuid::cuid().unwrap();
        let line = format!("INSERT INTO Class (id, matkulId, lecturerId, day, code, isAksel, taken, sessionId) VALUES ('{}', '{}', '{}', '{}', '{}', {}, {}, '{}');", id_class, class.matkul_id, class.lecture_id, class.day, class.code, false, 0, class.session_id);
        outfile.write_all(line.as_bytes()).await?;
        outfile.write_all(b"\n").await?;
    }
    Ok(())
}
