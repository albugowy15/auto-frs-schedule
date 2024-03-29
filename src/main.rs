use auto_frs_schedule::commands::{self, Commands};
use clap::Parser;

#[derive(Parser)]
#[command(version, author, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse cli command and args from struct
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
        Commands::Find {
            file,
            sheet,
            course,
        } => commands::find::find_handler(file, sheet, course).await?,
        Commands::Clean => {
            commands::clean::clean_handler().await?;
        }
        Commands::Sync => {
            commands::sync::sync_handler().await?;
        }
    }

    Ok(())
}
