mod db;
mod excel;
mod repo;
mod util;

use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::path::PathBuf;

use crate::{
    db::Connection,
    excel::Excel,
    repo::{Class, ClassRepository},
    util::write_output,
};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("Establish DB Connection");
    let db_url = env::var("FRS_HELPER_DB_URL").with_context(|| "FRS_HELPER_DB_URL must be set")?;
    let pool = Connection::create_connection(&db_url)
        .await
        .with_context(|| "Could not establish DB connection")?;

    println!("Get all subjects from DB");
    let subjects = ClassRepository::get_all_subject(&pool)
        .await
        .with_context(|| "Error retrieve all subjects from DB")?;

    println!("Get all lecturers from DB");
    let lecturers = ClassRepository::get_all_lecture(&pool)
        .await
        .with_context(|| "Error retrieve all lecturers from DB")?;

    println!("Get all sessions from DB");
    let sessions = ClassRepository::get_all_session(&pool)
        .await
        .with_context(|| "Error retrieve all sessions from DB")?;

    println!("Parse class schedule from Excel");
    let excel = Excel::new(&cli.file, &cli.sheet).with_context(|| {
        format!(
            "Error opening {} with sheet name '{:?}'",
            &cli.file.display(),
            &cli.sheet,
        )
    })?;

    let list_class: Vec<Class> = excel
        .parse_excel(&subjects, &lecturers, &sessions)
        .with_context(|| "Error parsing excel")?;

    if cli.push == true {
        println!("Insert {} classes to DB", list_class.len());
        ClassRepository::insert_classes(&pool, &list_class)
            .await
            .with_context(|| "Error inserting class to DB")?;
    }

    if let Some(path_output) = &cli.outdir {
        println!("Write {} classes to out directory", list_class.len());
        write_output(&path_output, &list_class)
            .await
            .with_context(|| "Error writing output to sql")?;
    }
    println!("Done");
    Ok(())
}
