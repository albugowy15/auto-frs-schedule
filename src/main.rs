use anyhow::{Context, Result};
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
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let bar = ProgressBar::new(100);
    bar.set_style(ProgressStyle::with_template(
        "{bar:50.cyan/blue} {pos}/{len} {msg}",
    )?);
    bar.set_message("Establish DB Connection");
    bar.inc(10);
    let db_url =
        env::var("FRS_HELPER_DB_URL").with_context(|| format!("FRS_HELPER_DB_URL must be set"))?;
    let mut db = Connection::create_connection(&db_url)
        .await
        .with_context(|| format!("Could not establish DB connection"))?;

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
            class_repo
                .get_all_subject(&mut db.conn)
                .await
                .with_context(|| format!("Error retrieve all subjects from DB"))?;
            bar.set_message("Get all lecturers from DB");
            bar.inc(10);
            class_repo
                .get_all_lecture(&mut db.conn)
                .await
                .with_context(|| format!("Error retrieve all lecturers from DB"))?;
            bar.set_message("Get all sessions from DB");
            bar.inc(10);
            class_repo
                .get_all_session(&mut db.conn)
                .await
                .with_context(|| format!("Error retrieve all sessions from DB"))?;
            bar.set_message("Parse class schedule from Excel");
            bar.inc(10);
            let excel = Excel::new(&path_to_excel, &sheet).with_context(|| {
                format!(
                    "Error opening {} with sheet name '{}'",
                    &path_to_excel, &sheet,
                )
            })?;
            let list_class: Vec<Class> = excel
                .parse_excel(
                    &class_repo.subjects,
                    &class_repo.lecturers,
                    &class_repo.sessions,
                )
                .with_context(|| format!("Error parsing excel"))?;
            let bar_msg = format!("Insert {} classes to DB", list_class.len());
            bar.set_message(bar_msg);
            bar.inc(10);
            class_repo
                .insert_data(&mut db.conn, list_class)
                .await
                .with_context(|| format!("Error inserting class to DB"))?;
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
            class_repo
                .get_all_subject(&mut db.conn)
                .await
                .with_context(|| format!("Error retrieve all subjects from DB"))?;
            bar.set_message("Get all lecturers from DB");
            bar.inc(10);
            class_repo
                .get_all_lecture(&mut db.conn)
                .await
                .with_context(|| format!("Error retrieve all lecturers from DB"))?;
            bar.set_message("Get all sessions from DB");
            bar.inc(10);
            class_repo
                .get_all_session(&mut db.conn)
                .await
                .with_context(|| format!("Error retrieve all sessions from DB"))?;
            bar.set_message("Parse class schedule from Excel");
            bar.inc(10);
            let excel = Excel::new(&path_to_excel, &sheet).with_context(|| {
                format!(
                    "Error opening {} with sheet name '{}'",
                    &path_to_excel, &sheet,
                )
            })?;
            let list_class: Vec<Class> = excel
                .parse_excel(
                    &class_repo.subjects,
                    &class_repo.lecturers,
                    &class_repo.sessions,
                )
                .with_context(|| format!("Error parsing excel"))?;
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
            write_output(&path_output, &list_class)
                .await
                .with_context(|| format!("Error writing output to sql"))?;
        }
        None => {
            println!("No command provided");
        }
    }
    bar.set_message("Closing DB Connection");
    bar.inc(10);
    db.close_connection()
        .await
        .with_context(|| "Error closing DB Connection")?;
    bar.set_message("Succesfully parsed excel");
    bar.inc(10);
    bar.finish();
    Ok(())
}

#[allow(deprecated)]
async fn write_output(path_output: &String, list_class: &Vec<Class>) -> Result<()> {
    let mut outfile = File::create(format!("{}/out.sql", path_output))
        .await
        .with_context(|| format!("Error create directory"))?;
    for class in list_class {
        let id_class = cuid::cuid().with_context(|| format!("Could not creat cuid"))?;
        let line = format!("INSERT INTO Class (id, matkulId, lecturerId, day, code, isAksel, taken, sessionId) VALUES ('{}', '{}', '{}', '{}', '{}', {}, {}, '{}');", id_class, class.matkul_id, class.lecture_id, class.day, class.code, false, 0, class.session_id);
        outfile
            .write_all(line.as_bytes())
            .await
            .with_context(|| format!("Error writing {} to file", line))?;
        outfile.write_all(b"\n").await?;
    }
    Ok(())
}
