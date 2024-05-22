use std::path::Path;

use anyhow::{Context, Result};
use tokio::{fs::File, io::AsyncWriteExt};

pub mod class_writer;
pub mod compare_writer;
pub mod sql_writer;

pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub async fn new(file_path: &Path) -> Result<Self> {
        Ok(Self {
            file: File::create(file_path).await?,
        })
    }

    async fn write(&mut self, query: String) -> Result<()> {
        self.file
            .write_all(query.as_bytes())
            .await
            .with_context(|| format!("Error writing to file: {:?}", self.file))?;
        Ok(())
    }

    async fn sync_all(&mut self) -> Result<()> {
        self.file
            .sync_all()
            .await
            .with_context(|| format!("Error syncing file: {:?}", self.file))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[tokio::test]
    async fn test_write() {
        let out_path = PathBuf::from("test_write.txt");
        let mut file_writer = FileWriter::new(&out_path).await.unwrap();

        // Write to the file
        let query = "SELECT * FROM users;".to_string();
        file_writer.write(query).await.unwrap();

        // Read the file
        let contents = tokio::fs::read_to_string("test_write.txt").await.unwrap();

        // Assert that the file contains the query
        assert_eq!(contents, "SELECT * FROM users;");

        // Clean up
        tokio::fs::remove_file("test_write.txt").await.unwrap();
    }
}
