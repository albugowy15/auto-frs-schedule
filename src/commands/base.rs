use anyhow::{Context, Result};
use clap::Subcommand;
use std::{collections::HashMap, path::PathBuf};

use crate::db::repository::{
    lecturer_repository::LecturerRepository, session_repository::SessionRepository,
    subject_repository::SubjectRepository,
};

#[derive(Subcommand)]
pub enum Commands {
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
    Clean,
}

pub async fn prepare_data(
    lecturer_repo: &LecturerRepository<'_>,
    subject_repo: &SubjectRepository<'_>,
    session_repo: &SessionRepository<'_>,
) -> Result<(
    HashMap<String, String>,
    HashMap<String, String>,
    HashMap<String, i8>,
)> {
    log::info!("Get all subjects from DB");

    let subjects = subject_repo
        .get_all_subject()
        .await
        .with_context(|| "Error retrieve all subjects from DB")?;

    log::info!("Get all lecturers from DB");
    let lecturers = lecturer_repo
        .get_all_lecture()
        .await
        .with_context(|| "Error retrieve all lecturers from DB")?;

    log::info!("Get all sessions from DB");
    let sessions = session_repo
        .get_all_session()
        .await
        .with_context(|| "Error retrieve all sessions from DB")?;

    Ok((subjects, lecturers, sessions))
}
