mod commands;
mod db;
mod utils;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use env_logger::{Builder, Env};
use std::env;
use std::path::PathBuf;

use crate::{
    commands::base::prepare_data, commands::compare::compare_handler,
    commands::update::update_handler, db::repository::class_repository::ClassRepository,
    db::Connection,
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
    let env = Env::default()
        .filter("AUTO_FRS_SCHEDULE_LOG_LEVEL")
        .write_style("AUTO_FRS_SCHEDULE_LOG_STYLE");
    env::set_var("AUTO_FRS_SCHEDULE_LOG_LEVEL", "INFO");
    env::set_var("AUTO_FRS_SCHEDULE_LOG_STYLE", "AUTO");
    Builder::from_env(env)
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .init();

    let cli = Cli::parse();

    log::info!("Establish DB Connection");
    let db_url = env::var("FRS_HELPER_DB_URL").with_context(|| "FRS_HELPER_DB_URL must be set")?;
    let pool = Connection::create_connection(&db_url)
        .await
        .with_context(|| "Could not establish DB connection")?;

    let class_repo = ClassRepository::new(&pool);
    let initial_class_data = prepare_data(&class_repo).await?;

    match &cli.command {
        Commands::Update {
            push,
            file,
            sheet,
            outdir,
        } => update_handler(push, file, sheet, outdir, initial_class_data, &class_repo).await?,
        Commands::Compare {
            file,
            sheet,
            outdir,
        } => compare_handler(file, sheet, outdir, &pool, initial_class_data).await?,
    }

    log::info!("Done");
    Ok(())
}
