use auto_frs_schedule::{
    db::Connection,
    excel::Excel,
    repo::{Class, ClassRepository},
};
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
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
    let cli = Cli::parse();
    let bar = ProgressBar::new(100);
    bar.set_style(ProgressStyle::with_template(
        "{bar:60.cyan/blue} {pos}/{len} {msg}",
    )?);
    bar.set_message("Establish DB Connection");
    bar.inc(10);
    let db_url = env::var("FRS_HELPER_DB_URL").expect("FRS_HELPER_DB_URL must be set");
    let mut db = Connection::create_connection(&db_url).await?;

    match &cli.command {
        Some(Commands::ExcelToDB { file, sheet }) => {
            bar.set_message("Opening excel file");
            bar.inc(20);
            let path_to_excel = match file {
                Some(path) => path
                    .to_str()
                    .expect("Error converting path to &str")
                    .to_string(),
                None => {
                    panic!("Error file path not found");
                }
            };
            let mut class_repo = ClassRepository::new();
            bar.set_message("Get all subjects from DB");
            bar.inc(10);
            class_repo.get_all_subject(&mut db.conn).await?;
            bar.set_message("Get all lecturers from DB");
            bar.inc(10);
            class_repo.get_all_lecture(&mut db.conn).await?;
            bar.set_message("Get all sessions from DB");
            bar.inc(10);
            class_repo.get_all_session(&mut db.conn).await?;
            bar.set_message("Parse class schedule from Excel");
            bar.inc(10);
            let excel = Excel::new(&path_to_excel, &sheet)?;
            let list_class: Vec<Class> = excel.parse_excel(
                &class_repo.subjects,
                &class_repo.lecturers,
                &class_repo.sessions,
            )?;
            let bar_msg = format!("Insert {} classes to DB", list_class.len());
            bar.set_message(bar_msg);
            bar.inc(10);
            class_repo.insert_data(&mut db.conn, list_class).await?;
        }
        Some(Commands::ParseExcel {
            file,
            sheet,
            outdir,
        }) => {
            bar.set_message("Opening excel file");
            bar.inc(10);
            let path_to_excel = match file {
                Some(path) => path
                    .to_str()
                    .expect("Error converting path to &str")
                    .to_string(),
                None => {
                    panic!("Error file path not found");
                }
            };
            let mut class_repo = ClassRepository::new();
            bar.set_message("Get all subjects from DB");
            bar.inc(10);
            class_repo.get_all_subject(&mut db.conn).await?;
            bar.set_message("Get all lecturers from DB");
            bar.inc(10);
            class_repo.get_all_lecture(&mut db.conn).await?;
            bar.set_message("Get all sessions from DB");
            bar.inc(10);
            class_repo.get_all_session(&mut db.conn).await?;
            bar.set_message("Parse class schedule from Excel");
            bar.inc(10);
            let excel = Excel::new(&path_to_excel, &sheet)?;
            let list_class: Vec<Class> = excel.parse_excel(
                &class_repo.subjects,
                &class_repo.lecturers,
                &class_repo.sessions,
            )?;
            bar.set_message("Open out directory");
            bar.inc(10);
            let path_output = match outdir {
                Some(path) => path
                    .to_str()
                    .expect("Error converting path output to str")
                    .to_string(),
                None => {
                    panic!("Error output path not found")
                }
            };
            bar.set_message("Write to out directory");
            bar.inc(10);
            write_output(&path_output, &list_class).await?;
        }
        None => {
            println!("No command provided");
        }
    }
    bar.set_message("Closing DB Connection");
    bar.inc(10);
    db.close_connection().await?;
    bar.set_message("Succesfully parsed excel");
    bar.inc(10);
    bar.finish();
    Ok(())
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
        let id_class = cuid::cuid().expect("Error creating cuid");
        let line = format!("INSERT INTO Class (id, matkulId, lecturerId, day, code, isAksel, taken, sessionId) VALUES ('{}', '{}', '{}', '{}', '{}', {}, {}, '{}');", id_class, class.matkul_id, class.lecture_id, class.day, class.code, false, 0, class.session_id);
        outfile.write_all(line.as_bytes()).await?;
        outfile.write_all(b"\n").await?;
    }
    Ok(())
}
