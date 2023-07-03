mod db;
mod excel;
mod repo;

use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::path::PathBuf;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::{
    db::Connection,
    excel::Excel,
    repo::{Class, ClassRepository},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "Optional to determine wether only parse or also push class to DB"
    )]
    push: Option<bool>,
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

    if let Some(_is_push) = &cli.push {
        println!("Insert {} classes to DB", list_class.len());
        ClassRepository::insert_data(&pool, &list_class)
            .await
            .with_context(|| "Error inserting class to DB")?;
    }

    if let Some(path_output) = &cli.outdir {
        println!("Write to out directory");
        write_output(&path_output, &list_class)
            .await
            .with_context(|| "Error writing output to sql")?;
    }
    println!("Done");
    Ok(())
}

#[allow(deprecated)]
async fn write_output(path_output: &PathBuf, list_class: &Vec<Class>) -> Result<()> {
    let outfile_path = path_output.join("out.sql");
    let mut outfile = BufWriter::new(
        File::create(outfile_path)
            .await
            .with_context(|| "Error creating output file")?,
    );
    let mut buffer = String::with_capacity(256);

    for class in list_class {
        let id_class = cuid::cuid().with_context(|| "Could not create cuid")?;
        buffer.clear();
        buffer.push_str("INSERT INTO Class (id, matkulId, lecturerId, day, code, isAksel, taken, sessionId) VALUES (\"");
        buffer.push_str(&id_class);
        buffer.push_str("\", \"");
        buffer.push_str(&class.matkul_id);
        buffer.push_str("\", \"");
        buffer.push_str(&class.lecture_id);
        buffer.push_str("\", \"");
        buffer.push_str(&class.day);
        buffer.push_str("\", \"");
        buffer.push_str(&class.code);
        buffer.push_str("\", ");
        buffer.push_str("false, 0, ");
        buffer.push_str(&class.session_id.to_string());
        buffer.push_str(");");
        buffer.push('\n');

        outfile
            .write_all(buffer.as_bytes())
            .await
            .with_context(|| format!("Error writing to file: {}", buffer))?;
    }

    outfile
        .flush()
        .await
        .with_context(|| "Error flushing output file")?;
    Ok(())
}
