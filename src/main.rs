use anyhow::{Context, Result};
use auto_frs_schedule::{
    db::Connection,
    excel::Excel,
    repo::{Class, ClassRepository},
};
use clap::Parser;
use std::env;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Parser)]
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
    let db_url =
        env::var("FRS_HELPER_DB_URL").with_context(|| format!("FRS_HELPER_DB_URL must be set"))?;
    let mut db = Connection::create_connection(&db_url)
        .await
        .with_context(|| format!("Could not establish DB connection"))?;

    println!("Get all subjects from DB");
    let subjects = ClassRepository::get_all_subject(&mut db.conn)
        .await
        .with_context(|| format!("Error retrieve all subjects from DB"))?;

    println!("Get all lecturers from DB");
    let lecturers = ClassRepository::get_all_lecture(&mut db.conn)
        .await
        .with_context(|| format!("Error retrieve all lecturers from DB"))?;

    println!("Get all sessions from DB");
    let sessions = ClassRepository::get_all_session(&mut db.conn)
        .await
        .with_context(|| format!("Error retrieve all sessions from DB"))?;

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
        .with_context(|| format!("Error parsing excel"))?;

    if let Some(_is_push) = &cli.push {
        println!("Insert {} classes to DB", list_class.len());
        ClassRepository::insert_data(&mut db.conn, &list_class)
            .await
            .with_context(|| format!("Error inserting class to DB"))?;
    }

    if let Some(path_output) = &cli.outdir {
        println!("Write to out directory");
        write_output(&path_output, &list_class)
            .await
            .with_context(|| format!("Error writing output to sql"))?;
    }

    println!("Closing DB Connection");
    db.close_connection()
        .await
        .with_context(|| "Error closing DB Connection")?;
    println!("Done");
    Ok(())
}

#[allow(deprecated)]
async fn write_output(path_output: &PathBuf, list_class: &Vec<Class>) -> Result<()> {
    let mut outfile = File::create(format!("{}/out.sql", path_output.display()))
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
