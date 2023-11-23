use anyhow::{Context, Result};
use sqlx::{MySql, Pool};
use std::collections::HashMap;

use crate::db::repository::{
    lecturer_repository::LecturerRepository, session_repository::SessionRepository,
    subject_repository::SubjectRepository, Repository,
};

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
