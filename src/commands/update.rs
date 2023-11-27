use std::path::PathBuf;

use anyhow::{Context, Result};
use sqlx::{MySql, Pool};

use crate::{
    commands::prepare_data,
    db::repository::{
        class_repository::{Class, ClassRepository},
        Repository,
    },
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

    let list_class: Vec<Class> = excel.get_schedule();
    let class_repo = ClassRepository::new(pool);

    if *push {
        log::info!("Insert {} classes to DB", list_class.len());
        class_repo
            .insert_classes(&list_class)
            .await
            .with_context(|| "Error inserting class to DB")?;
    }

    if let Some(path_output) = &outdir {
        log::info!("Write {} classes to out directory", list_class.len());
        let mut outfile = OutWriter::new(path_output).await?;
        outfile
            .write_output(&list_class)
            .await
            .with_context(|| "Error writing output to sql")?;
    }
    Ok(())
}
