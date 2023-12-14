mod commands;
mod db;
mod utils;

use crate::utils::env;
use anyhow::Result;
use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    env::setup_env();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Update {
            push,
            file,
            sheet,
            outdir,
        } => commands::update::update_handler(push, file, sheet, outdir).await?,
        Commands::Compare {
            file,
            sheet,
            outdir,
        } => commands::compare::compare_handler(file, sheet, outdir).await?,
        Commands::Clean => {
            commands::clean::clean_handler().await;
        }
        Commands::Sync => {
            commands::sync::sync_handler().await;
        }
    }

    log::info!("Done");
    Ok(())
}
