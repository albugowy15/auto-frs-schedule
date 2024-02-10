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
    let (mut db_classes, repo_data_res) =
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

    for excel_class in excel_classes {
        let key = (
            excel_class.subject_name.clone(),
            excel_class.class_code.clone(),
        );
        if let Some(db_class) = db_classes.get(&key) {
            let is_same_class = compare_class(db_class, &excel_class);
            if !is_same_class {
                changed.push((db_class.clone(), excel_class.clone()));
            }
            db_classes.remove(&key).unwrap();
        } else {
            added.push(excel_class);
        }
    }
    deleted.extend(db_classes.into_values());
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
