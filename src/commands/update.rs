use std::{path::PathBuf, sync::Arc};

use sqlx::MySqlPool;

use crate::{
    db::{
        self,
        repository::{
            class::{Class, ClassRepository},
            prepare_data, Repository,
        },
    },
    utils::{
        excel::{Excel, ScheduleParser},
        file::OutWriter,
    },
};

fn push_to_db(list_class: &Arc<Vec<Class>>, pool: &Arc<MySqlPool>) -> tokio::task::JoinHandle<()> {
    let cloned_list_class = Arc::clone(list_class);
    let cloned_pool = Arc::clone(pool);
    tokio::task::spawn(async move {
        log::info!("Insert {} classes to DB", cloned_list_class.len());
        ClassRepository::new(&cloned_pool)
            .insert_classes(&cloned_list_class)
            .await
            .expect("Error inserting class to DB");
    })
}

fn write_change_to_output_file(
    list_class: &Arc<Vec<Class>>,
    path_output: PathBuf,
) -> tokio::task::JoinHandle<()> {
    let cloned_list_class = Arc::clone(list_class);
    log::info!("Write {} classes to out directory", cloned_list_class.len());
    tokio::task::spawn(async move {
        OutWriter::new(&path_output)
            .await
            .expect("Cant create file")
            .write_output(&cloned_list_class)
            .await
            .expect("Error writing output to sql file");
    })
}

pub async fn update_handler(push: &bool, file: &PathBuf, sheet: &String, outdir: &Option<PathBuf>) {
    let pool = match db::Database::create_connection().await {
        Ok(pool) => Arc::new(pool),
        Err(e) => {
            log::error!("{}", e);
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
    let excel = match Excel::new(file, sheet) {
        Ok(excel) => excel.with_repo_data(repo_data),
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
        let handle = push_to_db(&list_class, &pool);
        handles.push(handle);
    }

    if let Some(path_output) = &outdir {
        let handle = write_change_to_output_file(&list_class, path_output.clone());
        handles.push(handle);
    }

    // join back all thread
    for handle in handles {
        if let Err(e) = handle.await {
            log::error!("Thread error: {}", e);
            return;
        };
    }
    pool.close().await;
}
