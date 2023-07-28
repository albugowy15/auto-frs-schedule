use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::repo::Class;

#[allow(deprecated)]
pub async fn write_output(path_output: &PathBuf, list_class: &Vec<Class>) -> Result<()> {
    let mut outfile = BufWriter::new(
        File::create(path_output.as_path())
            .await
            .with_context(|| "Error creating output file")?,
    );
    let mut buffer = String::with_capacity(256);

    for class in list_class {
        let id_class = cuid::cuid().with_context(|| "Could not create cuid")?;
        buffer.clear();
        buffer.push_str(
            "INSERT INTO Class (id, matkulId, day, code, isAksel, taken, sessionId) VALUES (\"",
        );
        buffer.push_str(&id_class);
        buffer.push_str("\", \"");
        buffer.push_str(&class.matkul_id);
        buffer.push_str("\", \"");
        buffer.push_str(&class.day);
        buffer.push_str("\", \"");
        buffer.push_str(&class.code);
        buffer.push_str("\", ");
        buffer.push_str("false, 0, ");
        buffer.push_str(&class.session_id.to_string());
        buffer.push_str(");");
        buffer.push('\n');
        outfile
            .write_all(buffer.as_bytes())
            .await
            .with_context(|| format!("Error writing to file: {}", buffer))?;
    }
    outfile
        .flush()
        .await
        .with_context(|| "Error flushing output file")?;
    Ok(())
}
