use crate::{
    db::{
        self,
        repository::{
            class::{ClassFromSchedule, ClassRepository},
            prepare_data, Repository,
        },
    },
    excel::Excel,
    file_writer::{compare_writer::CompareFileWriter, FileWriter},
    parser::schedule_parser::ScheduleParser,
};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub async fn compare_handler(file: &PathBuf, sheet: &str, outdir: &PathBuf) -> anyhow::Result<()> {
    let pool = Arc::new(db::Database::create_connection().await?);
    let class_repo = ClassRepository::new(&pool);
    let (db_classes, repo_data) = tokio::try_join!(class_repo.get_schedule(), prepare_data(&pool))?;

    let excel = Excel::new(file, sheet)?.with_repo_data(repo_data);
    let excel_classes = excel.get_schedule();

    println!(
        "Comparing {} classes from Excel with existing schedule",
        excel_classes.len()
    );
    let (added, deleted, changed) = compare_schedule(excel_classes, db_classes);
    if is_schedule_sync(&added, &deleted, &changed) {
        println!("No schedule change detected");
    } else {
        println!(
            "Detected {} changed, {} added, {} deleted class",
            changed.len(),
            added.len(),
            deleted.len()
        );
        println!("Write the result to {:?}", &outdir);
        let mut file_writer = FileWriter::new(outdir).await?;
        file_writer
            .write_compare_result(&added, &changed, &deleted)
            .await?;
    }

    pool.close().await;
    Ok(())
}

fn is_schedule_sync(
    added: &[ClassFromSchedule],
    changed: &[ClassFromSchedule],
    deleted: &[(ClassFromSchedule, ClassFromSchedule)],
) -> bool {
    added.is_empty() && changed.is_empty() && deleted.is_empty()
}

fn is_same_class(
    db_class: &ClassFromSchedule,
    excel_class: &ClassFromSchedule,
    cmp_lecs: bool,
) -> bool {
    db_class.subject_name == excel_class.subject_name
        && db_class.class_code == excel_class.class_code
        && db_class.session_start == excel_class.session_start
        && db_class.day == excel_class.day
        && cmp_lecs
}

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

    is_same_class(db_class, excel_class, cmp_lecs)
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
