mod db;
mod excel;
mod repo;
mod util;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;

use crate::{
    db::Connection,
    excel::{Excel, IntoMap, IntoStr},
    repo::{Class, ClassFromSchedule, ClassRepository},
    util::file::Writer,
};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compare {
        #[arg(short, long, value_name = "Required for latest schedule excel file")]
        file: PathBuf,

        #[arg(short, long, value_name = "Required for excel sheet name")]
        sheet: String,

        #[arg(short, long, value_name = "Required for output path")]
        outdir: PathBuf,
    },
    Update {
        #[arg(
            short,
            long,
            value_name = "Optional to determine wether only parse or also push class to DB"
        )]
        push: bool,

        #[arg(short, long, value_name = "Required for excel file path")]
        file: PathBuf,

        #[arg(short, long, value_name = "Required for excel sheet name")]
        sheet: String,

        #[arg(
            short,
            long,
            value_name = "Optional to write the sql statement to output directory"
        )]
        outdir: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let cli = Cli::parse();
    println!("Establish DB Connection");
    let db_url = env::var("FRS_HELPER_DB_URL").with_context(|| "FRS_HELPER_DB_URL must be set")?;
    let pool = Connection::create_connection(&db_url)
        .await
        .with_context(|| "Could not establish DB connection")?;

    println!("Get all subjects from DB");
    let class_repo = ClassRepository::new(&pool);
    let subjects = class_repo
        .get_all_subject()
        .await
        .with_context(|| "Error retrieve all subjects from DB")?;

    println!("Get all lecturers from DB");
    let lecturers = class_repo
        .get_all_lecture()
        .await
        .with_context(|| "Error retrieve all lecturers from DB")?;

    println!("Get all sessions from DB");
    let sessions = class_repo
        .get_all_session()
        .await
        .with_context(|| "Error retrieve all sessions from DB")?;

    match &cli.command {
        Commands::Update {
            push,
            file,
            sheet,
            outdir,
        } => {
            println!("Parse class schedule from Excel");
            let excel = Excel::new(&file, &sheet).with_context(|| {
                format!(
                    "Error opening {} with sheet name '{:?}'",
                    &file.display(),
                    &sheet,
                )
            })?;

            let list_class: Vec<Class> = excel
                .parse_excel(&subjects, &lecturers, &sessions)
                .with_context(|| "Error parsing excel")?;

            if *push == true {
                println!("Insert {} classes to DB", list_class.len());
                class_repo
                    .insert_classes(&list_class)
                    .await
                    .with_context(|| "Error inserting class to DB")?;
            }

            if let Some(path_output) = &outdir {
                println!("Write {} classes to out directory", list_class.len());
                let mut outfile = Writer::new(path_output).await?;
                outfile
                    .write_output(&list_class)
                    .await
                    .with_context(|| "Error writing output to sql")?;
            }
            println!("Done");
        }
        Commands::Compare {
            file,
            sheet,
            outdir,
        } => {
            let mut added: Vec<ClassFromSchedule> = Vec::new();
            let mut deleted: Vec<ClassFromSchedule> = Vec::new();
            let mut changed: Vec<(ClassFromSchedule, ClassFromSchedule)> = Vec::new();

            let class_repo = ClassRepository::new(&pool);
            let mut db_classes = class_repo.get_schedule().await?;
            let excel = Excel::new(&file, &sheet)?;
            let excel_classes = excel.updated_schedule_to_str(&subjects, &lecturers, &sessions);
            for class in excel_classes {
                let key = (class.subject_name.clone(), class.class_code.clone());
                match db_classes.get(&key) {
                    Some(val) => {
                        if !val.eq(&class) {
                            changed.push((val.clone(), class.clone()));
                        }
                        db_classes.remove(&key).unwrap();
                    }
                    None => {
                        added.push(class);
                    }
                }
            }
            if !db_classes.is_empty() {
                for (_, val) in db_classes.into_iter() {
                    deleted.push(val);
                }
            }
            let mut out_writer = Writer::new(&outdir).await?;
            out_writer
                .write_compare_result(&added, &changed, &deleted)
                .await
                .with_context(|| "Error writing output to sql")?;
        }
    }
    Ok(())
}
