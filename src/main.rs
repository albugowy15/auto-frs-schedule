mod commands;
mod db;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;
use commands::Commands;
use env_logger::{Builder, Env};
use std::env;

use crate::db::Connection;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

    match &cli.command {
        Commands::Update {
            push,
            file,
            sheet,
            outdir,
        } => commands::update::update_handler(push, file, sheet, outdir, &pool).await?,
        Commands::Compare {
            file,
            sheet,
            outdir,
        } => commands::compare::compare_handler(file, sheet, outdir, &pool).await?,
        Commands::Clean => {
            commands::clean::clean_handler(&pool).await;
        }
        Commands::Sync => {
            commands::sync::sync_handler(&pool).await;
        }
    }

    log::info!("Closing DB Connection");
    pool.close().await;

    log::info!("Done");
    Ok(())
}
