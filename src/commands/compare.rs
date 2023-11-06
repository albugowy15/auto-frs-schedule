use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use sqlx::{MySql, Pool};

use crate::shared::{
    excel::{Excel, GetScheduleUpdate},
    file::OutWriter,
    repo::{ClassFromSchedule, ClassRepository},
};

pub async fn compare_handler(
    file: &PathBuf,
    sheet: String,
    outdir: &PathBuf,
    pool: &Pool<MySql>,
    repo_data: (
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, i8>,
    ),
) -> Result<()> {
    let mut added: Vec<ClassFromSchedule> = Vec::new();
    let mut deleted: Vec<ClassFromSchedule> = Vec::new();
    let mut changed: Vec<(ClassFromSchedule, ClassFromSchedule)> = Vec::new();

    let class_repo = ClassRepository::new(&pool);
    println!("Get existing schedule from DB");
    let mut db_classes = class_repo
        .get_schedule()
        .await
        .with_context(|| "Error get schedules from DB")?;

    println!("Get latest schedule from Excel");
    let excel = Excel::new(&file, &sheet, repo_data.0, repo_data.1, repo_data.2)
        .with_context(|| "Error opening excel file")?;
    let excel_classes = excel.get_updated_schedule();

    println!(
        "Comparing {} classes from Excel with existing schedule",
        excel_classes.len()
    );
    for class in excel_classes {
        let key = (class.subject_name.clone(), class.class_code.clone());
        match db_classes.get(&key) {
            Some(val) => {
                if !val.eq(&class) {
                    changed.push((val.clone(), class.clone()));
                }
                db_classes.remove(&key).unwrap();
            }
            None => {
                added.push(class);
            }
        }
    }
    if !db_classes.is_empty() {
        for (_, val) in db_classes.into_iter() {
            deleted.push(val);
        }
    }
    println!(
        "Detected {} changed, {} added, {} deleted class",
        changed.len(),
        added.len(),
        deleted.len()
    );
    println!("Write the result to {:?}", &outdir);
    let mut out_writer = OutWriter::new(&outdir)
        .await
        .with_context(|| format!("Error creating {:?}", &outdir))?;
    out_writer
        .write_compare_result(&added, &changed, &deleted)
        .await
        .with_context(|| "Error writing result")?;

    Ok(())
}
