use std::path::PathBuf;

use crate::{
    db::{
        repository::{
            class::{ClassFromSchedule, ClassRepository},
            prepare_data, Repository,
        },
        Database,
    },
    utils::{
        excel::{Excel, ScheduleParser},
        file::OutWriter,
    },
};

pub async fn compare_handler(file: &PathBuf, sheet: &str, outdir: &PathBuf) {
    let pool = match Database::create_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            log::error!("Failed to create a db connection: {}", e);
            return;
        }
    };
    log::info!("Get existing schedule from DB");
    let class_repo = ClassRepository::new(&pool);
    let (mut db_classes_res, repo_data_res) =
        match tokio::try_join!(class_repo.get_schedule(), prepare_data(&pool)) {
            Ok((db_classes_res, repo_data_res)) => (db_classes_res, repo_data_res),
            Err(e) => {
                log::error!("Error getting schedule: {}", e);
                return;
            }
        };

    log::info!("Get latest schedule from Excel");
    let excel = match Excel::new(file, sheet, repo_data_res) {
        Ok(excel) => excel,
        Err(e) => {
            log::error!("Error opening excel file: {}", e);
            return;
        }
    };
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
                let mut db_lec_codes = val.lecturer_code.clone();
                db_lec_codes.sort();
                let mut excel_lec_codes = class.lecturer_code.clone();
                excel_lec_codes.sort();
                if !db_lec_codes.eq(&excel_lec_codes) {
                    changed.push((val.clone(), class.clone()));
                }
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
    if let Err(e) = OutWriter::new(outdir)
        .await
        .unwrap()
        .write_compare_result(&added, &changed, &deleted)
        .await
    {
        log::error!("Error writing result: {}", e);
        return;
    };

    pool.close().await;
}
