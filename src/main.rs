use auto_frs_schedule::{
    commands::{self, Commands},
    utils::env,
};
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    // setup env for logger and rust backtrace
    env::setup_env();

    // parse cli command and args from struct
    let cli = Cli::parse();

    match &cli.command {
        Commands::Update {
            push,
            file,
            sheet,
            outdir,
        } => commands::update::update_handler(push, file, sheet, outdir).await,
        Commands::Compare {
            file,
            sheet,
            outdir,
        } => commands::compare::compare_handler(file, sheet, outdir).await,
        Commands::Clean => {
            commands::clean::clean_handler().await;
        }
        Commands::Sync => {
            commands::sync::sync_handler().await;
        }
    }

    log::info!("Done");
}
