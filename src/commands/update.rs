use sqlx::MySqlPool;
use std::{path::PathBuf, sync::Arc};

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

fn push_to_db(
    list_class: &Arc<Vec<Class>>,
    pool: &Arc<MySqlPool>,
) -> tokio::task::JoinHandle<anyhow::Result<()>> {
    let cloned_list_class = Arc::clone(list_class);
    let cloned_pool = Arc::clone(pool);
    tokio::task::spawn(async move {
        println!("Insert {} classes to DB", cloned_list_class.len());
        ClassRepository::new(&cloned_pool)
            .insert_classes(&cloned_list_class)
            .await?;
        Ok(())
    })
}

fn write_change_to_output_file(
    list_class: &Arc<Vec<Class>>,
    path_output: PathBuf,
) -> tokio::task::JoinHandle<anyhow::Result<()>> {
    let cloned_list_class = Arc::clone(list_class);
    println!("Write {} classes to out directory", cloned_list_class.len());
    tokio::task::spawn(async move {
        OutWriter::new(&path_output)
            .await?
            .write_output(&cloned_list_class)
            .await?;
        Ok(())
    })
}

pub async fn update_handler(
    push: &bool,
    file: &PathBuf,
    sheet: &str,
    outdir: &Option<PathBuf>,
) -> anyhow::Result<()> {
    println!("Open db connection...");
    let pool = Arc::new(db::Database::create_connection().await?);
    let repo_data = prepare_data(&pool).await?;

    println!("Parse class schedule from Excel");
    let excel = Excel::new(file, sheet)?.with_repo_data(repo_data);
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
        handle.await??;
    }
    pool.close().await;
    println!("Done");
    Ok(())
}
