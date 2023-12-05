use std::path::PathBuf;

use anyhow::{Context, Result};
use sqlx::{MySql, Pool};

use crate::{
    commands::prepare_data,
    db::repository::{
        class_repository::{ClassFromSchedule, ClassRepository},
        Repository,
    },
    utils::{
        excel::{Excel, ScheduleParser},
        file::OutWriter,
    },
};

pub async fn compare_handler(
    file: &PathBuf,
    sheet: &str,
    outdir: &PathBuf,
    pool: &Pool<MySql>,
) -> Result<()> {
    log::info!("Get existing schedule from DB");
    let class_repo = ClassRepository::new(pool);
    let (mut db_classes_res, repo_data_res) =
        tokio::try_join!(class_repo.get_schedule(), prepare_data(pool)).map_err(|e| {
            log::error!("Error getting schedule: {}", e);
            e
        })?;

    log::info!("Get latest schedule from Excel");
    let excel = Excel::new(
        file,
        sheet,
        repo_data_res.0,
        repo_data_res.1,
        repo_data_res.2,
    )
    .with_context(|| "Error opening excel file")?;
    let excel_classes: Vec<ClassFromSchedule> = excel.get_schedule();

    log::info!(
        "Comparing {} classes from Excel with existing schedule",
        excel_classes.len()
    );
    let (mut added, mut deleted, mut changed) = (Vec::new(), Vec::new(), Vec::new());

    for class in excel_classes {
        let key = (class.subject_name.clone(), class.class_code.clone());
        if let Some(val) = db_classes_res.get(&key) {
            if !val.eq(&class) {
                changed.push((val.clone(), class.clone()));
            }
            db_classes_res.remove(&key).unwrap();
        } else {
            added.push(class);
        }
    }
    deleted.extend(db_classes_res.into_values());
    log::info!(
        "Detected {} changed, {} added, {} deleted class",
        changed.len(),
        added.len(),
        deleted.len()
    );
    log::info!("Write the result to {:?}", &outdir);
    OutWriter::new(outdir)
        .await
        .with_context(|| format!("Error creating {:?}", outdir))?
        .write_compare_result(&added, &changed, &deleted)
        .await
        .with_context(|| "Error writing result")?;

    Ok(())
}
