use std::{path::PathBuf, sync::Arc};

use crate::{
    db::{
        repository::{class::ClassRepository, prepare_data, Repository},
        Connection,
    },
    utils::{
        excel::{Excel, ScheduleParser},
        file::OutWriter,
    },
};

pub async fn update_handler(push: &bool, file: &PathBuf, sheet: &String, outdir: &Option<PathBuf>) {
    let pool = match Connection::create_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            log::error!("Failed to create a db connection: {}", e);
            return;
        }
    };
    let repo_data = match prepare_data(&pool).await {
        Ok(repo_data) => repo_data,
        Err(e) => {
            log::error!("Failed prepare initial data: {}", e);
            return;
        }
    };
    log::info!("Parse class schedule from Excel");
    let excel = match Excel::new(file, sheet, repo_data) {
        Ok(excel) => excel,
        Err(e) => {
            log::error!(
                "Error opening {} with sheet name '{:?}': {}",
                &file.display(),
                &sheet,
                e
            );
            return;
        }
    };

    let list_class = Arc::new(excel.get_schedule());
    let mut handles = Vec::new();

    if *push {
        let cloned_list_class = Arc::clone(&list_class);
        let cloned_pool = pool.clone();
        let handle = tokio::spawn(async move {
            log::info!("Insert {} classes to DB", cloned_list_class.len());
            ClassRepository::new(&cloned_pool)
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
            OutWriter::new(&cloned_path_output)
                .await
                .expect("Cant create file")
                .write_output(&cloned_list_class)
                .await
                .expect("Error writing output to sql file");
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Err(e) = handle.await {
            log::error!("Thread error: {}", e);
            return;
        };
    }
    pool.close().await;
}
