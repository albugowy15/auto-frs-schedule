use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use clap::Subcommand;
use sqlx::{MySql, Pool};

use crate::db::repository::{
    lecturer_repository::LecturerRepository, session_repository::SessionRepository,
    subject_repository::SubjectRepository, Repository,
};

pub mod clean;
pub mod compare;
pub mod sync;
pub mod update;

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
    Sync,
}

pub async fn prepare_data(
    pool: &Pool<MySql>,
) -> Result<(
    HashMap<String, String>,
    HashMap<String, String>,
    HashMap<String, i8>,
)> {
    log::info!("Get all subjects from DB");
    let lecturer_repo = LecturerRepository::new(pool);
    let subject_repo = SubjectRepository::new(pool);
    let session_repo = SessionRepository::new(pool);
    let (subjects, lecturers, sessions) = tokio::try_join!(
        subject_repo.get_all_subjects(),
        lecturer_repo.get_all_lecturers(),
        session_repo.get_all_sessions()
    )
    .map_err(|e| {
        log::error!(
            "Error retrieve subjects, lecturers, and session from DB: {}",
            e
        );
        e
    })?;
    Ok((subjects, lecturers, sessions))
}
