use std::path::PathBuf;

use clap::Subcommand;
use sqlx::MySqlPool;

use crate::db::Database;

pub mod clean;
pub mod compare;
pub mod find;
pub mod sync;
pub mod update;

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        long_about = "Compares the class schedule stored in the database with the latest data from an Excel file."
    )]
    Compare {
        #[arg(short, long, value_name = "Required for latest schedule excel file")]
        file: PathBuf,

        #[arg(short, long, value_name = "Required for excel sheet name")]
        sheet: String,

        #[arg(short, long, value_name = "Required for output path")]
        outdir: PathBuf,
    },
    #[command(
        long_about = "Parses all class data from an Excel file and subsequently updates the MySQL database. Alternatively, it provides an option to save the parsed data to an SQL file."
    )]
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
    #[command(
        long_about = "Removes any invalid foreign keys present in the _ClassToPlan and _ClassToLecturer tables."
    )]
    #[command(long_about = "Find class schedule from excel")]
    Find {
        #[arg(short, long, value_name = "Required for excel file path")]
        file: PathBuf,

        #[arg(short, long, value_name = "Required for excel sheet name")]
        sheet: String,

        #[arg(short, long, value_name = "Required for class subject name to find")]
        course: String,
    },
    Clean,
    #[command(
        long_about = "Synchronizes the taken field in the Class table and the totalSks field in the Plan table to reflect their current values."
    )]
    Sync,
}

async fn create_db_connection() -> anyhow::Result<MySqlPool> {
    let pool = Database::create_connection()
        .await
        .map_err::<anyhow::Error, _>(|e| {
            log::error!("Failed to create a db connection: {}", e);
            e
        })?;

    Ok(pool)
}
