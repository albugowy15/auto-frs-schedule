use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::repo::Class;

#[allow(deprecated)]
pub async fn write_output(path_output: &PathBuf, list_class: &Vec<Class>) -> Result<()> {
    let mut outfile = File::create(path_output.as_path()).await?;

    for class in list_class {
        let id_class = cuid::cuid().with_context(|| "Could not create cuid")?;
        let query = format!(
            "INSERT INTO Class (id, matkulId, day, code, isAksel, taken, sessionId) VALUES ('{}', '{}', '{}', '{}', false, 0, {});\n",
            id_class,
            class.matkul_id,
            class.day,
            class.code,
            class.session_id
        );
        outfile
            .write_all(query.as_bytes())
            .await
            .with_context(|| format!("Error writing to file: {}", query))?;
    }
    outfile.sync_all().await?;
    Ok(())
}
