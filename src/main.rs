use auto_frs_schedule::{
    db::Connection,
    excel::Excel,
    repo::{Class, ClassRepository},
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::{env, error::Error};
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
    let db_url = env::var("FRS_HELPER_DB_URL").expect("FRS_HELPER_DB_URL must be set");
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
            let mut class_repo = ClassRepository::new();
            class_repo.get_all_subject(&mut db.conn).await?;
            class_repo.get_all_lecture(&mut db.conn).await?;
            class_repo.get_all_session(&mut db.conn).await?;
            let list_class = parse_excel(&path_to_excel, &sheet, &class_repo)?;
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
            let mut class_repo = ClassRepository::new();
            class_repo.get_all_subject(&mut db.conn).await?;
            class_repo.get_all_lecture(&mut db.conn).await?;
            class_repo.get_all_session(&mut db.conn).await?;
            let list_class = parse_excel(&path_to_excel, &sheet, &class_repo)?;
            let path_output = match outdir {
                Some(path) => path.to_str().unwrap().to_string(),
                None => {
                    panic!("Error output path not found")
                }
            };
            write_output(&path_output, &list_class).await?;
        }
        None => {
            println!("No command provided");
        }
    }
    db.close_connection().await?;
    Ok(())
}

fn parse_excel(
    path_to_excel: &String,
    sheet: &String,
    sql_data: &ClassRepository,
) -> Result<Vec<Class>, Box<dyn Error>> {
    let excel = Excel::new(&path_to_excel, &sheet)?;
    let list_class =
        excel.parse_excel(&sql_data.subjects, &sql_data.lecturers, &sql_data.sessions)?;
    Ok(list_class)
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
    println!("Successfully write out.sql at {}", path_output);
    Ok(())
}
