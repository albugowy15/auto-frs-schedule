use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use sqlx::{MySql, Pool};

use crate::{
    commands::prepare_data,
    db::repository::{class_repository::ClassRepository, Repository},
    utils::{
        excel::{Excel, ScheduleParser},
        file::OutWriter,
    },
};

pub async fn update_handler(
    push: &bool,
    file: &PathBuf,
    sheet: &String,
    outdir: &Option<PathBuf>,
    pool: &Pool<MySql>,
) -> Result<()> {
    let repo_data = prepare_data(pool).await?;
    log::info!("Parse class schedule from Excel");
    let excel =
        Excel::new(file, sheet, repo_data.0, repo_data.1, repo_data.2).with_context(|| {
            format!(
                "Error opening {} with sheet name '{:?}'",
                &file.display(),
                &sheet,
            )
        })?;

    let list_class = Arc::new(excel.get_schedule());
    let mut handles = Vec::new();

    if *push {
        let cloned_list_class = Arc::clone(&list_class);
        let cloned_pool = pool.clone();
        let handle = tokio::task::spawn(async move {
            log::info!("Insert {} classes to DB", cloned_list_class.len());
            let class_repo = ClassRepository::new(&cloned_pool);
            class_repo
                .insert_classes(&cloned_list_class)
                .await
                .expect("Error inserting class to DB");
        });
        handles.push(handle);
    }

    if let Some(path_output) = &outdir {
        let cloned_list_class = Arc::clone(&list_class);
        let cloned_path_output = path_output.clone();
        log::info!("Write {} classes to out directory", cloned_list_class.len());
        let handle = tokio::task::spawn(async move {
            let mut outfile = OutWriter::new(&cloned_path_output)
                .await
                .expect("Cant create file");
            outfile
                .write_output(&cloned_list_class)
                .await
                .expect("Error writing output to sql file");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await?;
    }
    Ok(())
}
