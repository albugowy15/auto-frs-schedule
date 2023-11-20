use anyhow::{Context, Result};
use std::collections::HashMap;

use crate::db::repository::class_repository::ClassRepository;

pub async fn prepare_data(
    class_repo: &ClassRepository<'_>,
) -> Result<(
    HashMap<String, String>,
    HashMap<String, String>,
    HashMap<String, i8>,
)> {
    log::info!("Get all subjects from DB");

    let subjects = class_repo
        .get_all_subject()
        .await
        .with_context(|| "Error retrieve all subjects from DB")?;

    log::info!("Get all lecturers from DB");
    let lecturers = class_repo
        .get_all_lecture()
        .await
        .with_context(|| "Error retrieve all lecturers from DB")?;

    log::info!("Get all sessions from DB");
    let sessions = class_repo
        .get_all_session()
        .await
        .with_context(|| "Error retrieve all sessions from DB")?;

    Ok((subjects, lecturers, sessions))
}
