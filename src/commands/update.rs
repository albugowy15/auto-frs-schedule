use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};

use crate::{
    db::repository::class_repository::{Class, ClassRepository},
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
    repo_data: (
        HashMap<String, String>,
        HashMap<String, String>,
        HashMap<String, i8>,
    ),
    class_repo: &ClassRepository<'_>,
) -> Result<()> {
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
