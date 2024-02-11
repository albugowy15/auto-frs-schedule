use std::{collections::HashMap, path::PathBuf, sync::Arc};

use anyhow::{Error, Result};
use sqlx::MySqlPool;

use crate::{
    commands::create_db_connection,
    db::repository::{
        class::{ClassFromSchedule, ClassRepository},
        prepare_data, LecturerSubjectSessionMap, Repository,
    },
    utils::{
        excel::{Excel, ScheduleParser},
        file::OutWriter,
    },
};

fn compare_class(db_class: &ClassFromSchedule, excel_class: &ClassFromSchedule) -> bool {
    let db_class_lec = &db_class.lecturer_code;
    let excel_class_lec = &excel_class.lecturer_code;

    if db_class_lec.len() != excel_class_lec.len() {
        return false;
    }

    let mut db_lec_sorted = db_class_lec.clone();
    let mut excel_lec_sorted = excel_class_lec.clone();
    db_lec_sorted.sort();
    excel_lec_sorted.sort();

    let cmp_lecs = db_lec_sorted.eq(&excel_lec_sorted);

    db_class.subject_name == excel_class.subject_name
        && db_class.class_code == excel_class.class_code
        && db_class.session_start == excel_class.session_start
        && db_class.day == excel_class.day
        && cmp_lecs
}

type SpawnGetSchedule =
    tokio::task::JoinHandle<Result<HashMap<(String, String), ClassFromSchedule>, Error>>;
fn spawn_get_schedule(pool: &Arc<MySqlPool>) -> SpawnGetSchedule {
    let cloned_pool = pool.clone();
    tokio::task::spawn(async move { ClassRepository::new(&cloned_pool).get_schedule().await })
}

type SpawnPrepareData = tokio::task::JoinHandle<Result<LecturerSubjectSessionMap, Error>>;
fn spawn_prepare_data(pool: &Arc<MySqlPool>) -> SpawnPrepareData {
    let cloned_pool = pool.clone();
    tokio::task::spawn(async move { prepare_data(&cloned_pool).await })
}

fn compare_schedule(
    excel_classes: Vec<ClassFromSchedule>,
    mut db_classes: HashMap<(String, String), ClassFromSchedule>,
) -> (
    Vec<ClassFromSchedule>,
    Vec<ClassFromSchedule>,
    Vec<(ClassFromSchedule, ClassFromSchedule)>,
) {
    let (mut added, mut deleted, mut changed) = (Vec::new(), Vec::new(), Vec::new());

    for excel_class in excel_classes.into_iter() {
        let key = (
            excel_class.subject_name.clone(),
            excel_class.class_code.clone(),
        );
        if let Some(db_class) = db_classes.remove(&key) {
            let is_same_class = compare_class(&db_class, &excel_class);
            if !is_same_class {
                changed.push((db_class, excel_class));
            }
        } else {
            added.push(excel_class);
        }
    }
    deleted.extend(db_classes.into_values());

    (added, deleted, changed)
}

pub async fn compare_handler(file: &PathBuf, sheet: &str, outdir: &PathBuf) {
    let pool = Arc::new(create_db_connection().await.unwrap());
    log::info!("Get existing schedule from DB");

    let class_repo_schedule_task = spawn_get_schedule(&pool);
    let prepare_data_task = spawn_prepare_data(&pool);
    let (db_classes_res, repo_data_res) =
        tokio::try_join!(class_repo_schedule_task, prepare_data_task)
            .map_err(|e| {
                log::error!("Thread error: {}", e);
            })
            .unwrap();

    let db_classes = match db_classes_res {
        Ok(res) => res,
        Err(e) => {
            log::error!("Error getting db classes: {}", e);
            return;
        }
    };

    let repo_data = match repo_data_res {
        Ok(res) => res,
        Err(e) => {
            log::error!("Error preparing data: {}", e);
            return;
        }
    };

    log::info!("Get latest schedule from Excel");
    let excel = match Excel::new(file, sheet) {
        Ok(excel) => excel.with_repo_data(repo_data),
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
    let (added, deleted, changed) = compare_schedule(excel_classes, db_classes);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_two_same_class() {
        let db_class = ClassFromSchedule {
            subject_name: String::from("Pemrograman Web"),
            class_code: String::from("A"),
            lecturer_code: vec!["AD".to_string(), "AZ".to_string()],
            day: String::from("Senin"),
            session_start: String::from("13.00"),
        };

        let excel_class = ClassFromSchedule {
            subject_name: String::from("Pemrograman Web"),
            class_code: String::from("A"),
            lecturer_code: vec!["AD".to_string(), "AZ".to_string()],
            day: String::from("Senin"),
            session_start: String::from("13.00"),
        };

        assert!(compare_class(&db_class, &excel_class));
    }

    #[test]
    fn test_compare_two_diff_class() {
        let db_class = ClassFromSchedule {
            subject_name: String::from("Pemrograman Jaringan"),
            class_code: String::from("A"),
            lecturer_code: vec!["AZ".to_string(), "AD".to_string()],
            day: String::from("Senin"),
            session_start: String::from("13.00"),
        };

        let excel_class = ClassFromSchedule {
            subject_name: String::from("Pemrograman Web"),
            class_code: String::from("A"),
            lecturer_code: vec!["AD".to_string(), "AZ".to_string()],
            day: String::from("Senin"),
            session_start: String::from("13.00"),
        };

        assert!(!compare_class(&db_class, &excel_class));
    }

    #[test]
    fn test_compare_two_same_class_with_diff_lecs() {
        let db_class = ClassFromSchedule {
            subject_name: String::from("Pemrograman Web"),
            class_code: String::from("A"),
            lecturer_code: vec!["AZ".to_string(), "AD".to_string()],
            day: String::from("Senin"),
            session_start: String::from("13.00"),
        };

        let excel_class = ClassFromSchedule {
            subject_name: String::from("Pemrograman Web"),
            class_code: String::from("A"),
            lecturer_code: vec!["AD".to_string(), "AZ".to_string()],
            day: String::from("Senin"),
            session_start: String::from("13.00"),
        };

        assert!(compare_class(&db_class, &excel_class));
    }
}
