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
    let mut added: Vec<ClassFromSchedule> = Vec::new();
    let mut deleted: Vec<ClassFromSchedule> = Vec::new();
    let mut changed: Vec<(ClassFromSchedule, ClassFromSchedule)> = Vec::new();

    let class_repo = ClassRepository::new(pool);
    log::info!("Get existing schedule from DB");
    let db_classes = class_repo.get_schedule();
    let repo_data = prepare_data(pool);
    let result = tokio::try_join!(db_classes, repo_data);
    match result {
        Ok((mut db_classes_res, repo_data_res)) => {
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

            for class in excel_classes {
                let key = (class.subject_name.clone(), class.class_code.clone());
                match db_classes_res.get(&key) {
                    Some(val) => {
                        if !val.eq(&class) {
                            changed.push((val.clone(), class.clone()));
                        }
                        db_classes_res.remove(&key).unwrap();
                    }
                    None => {
                        added.push(class);
                    }
                }
            }
            if !db_classes_res.is_empty() {
                for (_, val) in db_classes_res.into_iter() {
                    deleted.push(val);
                }
            }
            log::info!(
                "Detected {} changed, {} added, {} deleted class",
                changed.len(),
                added.len(),
                deleted.len()
            );
            log::info!("Write the result to {:?}", &outdir);
            let mut out_writer = OutWriter::new(outdir)
                .await
                .with_context(|| format!("Error creating {:?}", &outdir))?;
            out_writer
                .write_compare_result(&added, &changed, &deleted)
                .await
                .with_context(|| "Error writing result")?;
        }
        Err(e) => {
            log::error!("Error getting schedule: {}", e);
        }
    };

    Ok(())
}
