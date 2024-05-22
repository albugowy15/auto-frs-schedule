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
    excel::Excel,
    file_writer::{sql_writer::SqlFileWriter, FileWriter},
    parser::schedule_parser::ScheduleParser,
};

pub async fn update_handler(
    push: &bool,
    file: &PathBuf,
    sheet: &str,
    outdir: &Option<PathBuf>,
) -> anyhow::Result<()> {
    let pool = Arc::new(db::Database::create_connection().await?);
    let repo_data = prepare_data(&pool).await?;

    println!("Parse class schedule from Excel");
    let excel = Excel::new(file, sheet)?.with_repo_data(repo_data);
    let list_class = Arc::new(excel.get_schedule());
    let mut handles = Vec::new();

    if *push {
        let handle = tokio::task::spawn(push_to_db(list_class.clone(), pool.clone()));
        handles.push(handle);
    }

    if let Some(path_output) = &outdir {
        let handle = tokio::task::spawn(write_change_to_output_file(
            list_class.clone(),
            path_output.clone(),
        ));
        handles.push(handle);
    }

    // join back all thread
    for handle in handles {
        handle.await??;
    }
    pool.close().await;
    println!("Closing database connection");
    println!("Done");
    Ok(())
}

async fn push_to_db(list_class: Arc<Vec<Class>>, pool: Arc<MySqlPool>) -> anyhow::Result<()> {
    println!("Insert {} classes to DB", list_class.len());
    ClassRepository::new(&pool)
        .insert_classes(&list_class)
        .await?;
    Ok(())
}

async fn write_change_to_output_file(
    list_class: Arc<Vec<Class>>,
    path_output: PathBuf,
) -> anyhow::Result<()> {
    println!("Write {} classes to out directory", list_class.len());
    FileWriter::new(&path_output)
        .await?
        .write_sql(&list_class)
        .await?;
    Ok(())
}
