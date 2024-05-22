use anyhow::Context;

use crate::db::repository::class::Class;

use super::FileWriter;

pub trait SqlFileWriter {
    fn write_sql(
        &mut self,
        list_class: &[Class],
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

impl SqlFileWriter for FileWriter {
    #[allow(deprecated)]
    async fn write_sql(&mut self, list_class: &[Class]) -> anyhow::Result<()> {
        for class in list_class.iter() {
            let id_class = cuid::cuid().with_context(|| "Could not create cuid")?;
            let query = format!(
            "INSERT INTO Class (id, matkulId, day, code, isAksel, taken, sessionId) VALUES ('{}', '{}', '{}', '{}', false, 0, {});\n",
            id_class,
            class.matkul_id,
            class.day,
            class.code,
            class.session_id
        );
            self.write(query).await?;
        }
        self.sync_all().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_write_output() {
        let out_path = PathBuf::from("test_write_output.txt");
        let mut file_writer = FileWriter::new(&out_path).await.unwrap();

        // Create some test data
        let class = Class {
            matkul_id: "CS101".to_string(),
            day: "Monday".to_string(),
            code: "C1".to_string(),
            session_id: 1,
            lecturers_id: vec![],
        };

        let list_class = vec![class];

        // Write the output to the file
        file_writer.write_sql(&list_class).await.unwrap();

        // Read the file
        let contents = tokio::fs::read_to_string("test_write_output.txt")
            .await
            .unwrap();

        // Assert that the file contains the output
        assert!(contents.contains(
            "INSERT INTO Class (id, matkulId, day, code, isAksel, taken, sessionId) VALUES ("
        ));
        assert!(contents.contains("'CS101', 'Monday', 'C1', false, 0, 1"));

        // Clean up
        tokio::fs::remove_file("test_write_output.txt")
            .await
            .unwrap();
    }
}
