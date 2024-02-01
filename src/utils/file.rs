use std::path::Path;

use anyhow::{Context, Result};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::db::repository::class::{Class, ClassFromSchedule};

enum CompareVecResult<'a> {
    DBAndExcel(&'a Vec<(ClassFromSchedule, ClassFromSchedule)>),
    Excel(&'a Vec<ClassFromSchedule>),
}

pub struct OutWriter {
    file: File,
}

impl OutWriter {
    pub async fn new(out_path: &Path) -> Result<Self> {
        Ok(Self {
            file: File::create(out_path).await?,
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

    async fn write_class_info(&mut self, prefix: &str, class: &ClassFromSchedule) -> Result<()> {
        let query = format!(
            "{} {} {}, {} {}, {:?}\n",
            prefix,
            class.subject_name,
            class.class_code,
            class.day,
            class.session_start,
            class.lecturer_code
        );
        self.write(query).await?;
        Ok(())
    }

    async fn write_change(&mut self, header: &str, result: &CompareVecResult<'_>) -> Result<()> {
        let header_section = format!("--- {} ---\n", header);
        self.write(header_section).await?;
        match result {
            CompareVecResult::DBAndExcel(data) => {
                for (db_class, excel_class) in data.iter() {
                    self.write_class_info("From DB :", db_class).await?;
                    self.write_class_info("From Excel :", excel_class).await?;
                }
            }
            CompareVecResult::Excel(data) => {
                for class in data.iter() {
                    self.write_class_info("", class).await?;
                }
            }
        };
        Ok(())
    }

    #[allow(deprecated)]
    pub async fn write_output(&mut self, list_class: &[Class]) -> Result<()> {
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

    pub async fn write_compare_result(
        &mut self,
        added: &Vec<ClassFromSchedule>,
        changed: &Vec<(ClassFromSchedule, ClassFromSchedule)>,
        deleted: &Vec<ClassFromSchedule>,
    ) -> Result<()> {
        self.write_change("ADDED", &CompareVecResult::Excel(added))
            .await?;
        self.write_change("CHANGED", &CompareVecResult::DBAndExcel(changed))
            .await?;
        self.write_change("DELETED", &CompareVecResult::Excel(deleted))
            .await?;
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
        let mut out_writer = OutWriter::new(&out_path).await.unwrap();

        // Write to the file
        let query = "SELECT * FROM users;".to_string();
        out_writer.write(query).await.unwrap();

        // Read the file
        let contents = tokio::fs::read_to_string("test_write.txt").await.unwrap();

        // Assert that the file contains the query
        assert_eq!(contents, "SELECT * FROM users;");

        // Clean up
        tokio::fs::remove_file("test_write.txt").await.unwrap();
    }

    #[tokio::test]
    async fn test_write_change() {
        let out_path = PathBuf::from("test_write_change.txt");
        let mut out_writer = OutWriter::new(&out_path).await.unwrap();

        // Create some test data
        let class1 = ClassFromSchedule {
            subject_name: "Math".to_string(),
            class_code: "M101".to_string(),
            day: "Monday".to_string(),
            session_start: "08:00".to_string(),
            lecturer_code: vec!["L1".to_string()],
        };
        let class2 = ClassFromSchedule {
            subject_name: "Physics".to_string(),
            class_code: "P101".to_string(),
            day: "Tuesday".to_string(),
            session_start: "09:00".to_string(),
            lecturer_code: vec!["L2".to_string()],
        };
        let data = vec![(class1, class2)];

        // Write the change to the file
        out_writer
            .write_change("CHANGED", &CompareVecResult::DBAndExcel(&data))
            .await
            .unwrap();

        // Read the file
        let contents = tokio::fs::read_to_string("test_write_change.txt")
            .await
            .unwrap();

        // Assert that the file contains the change
        assert!(contents.contains("--- CHANGED ---"));
        assert!(contents.contains("From DB : Math M101, Monday 08:00, [\"L1\"]"));
        assert!(contents.contains("From Excel : Physics P101, Tuesday 09:00, [\"L2\"]"));

        // Clean up
        tokio::fs::remove_file("test_write_change.txt")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_write_output() {
        let out_path = PathBuf::from("test_write_output.txt");
        let mut out_writer = OutWriter::new(&out_path).await.unwrap();

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
        out_writer.write_output(&list_class).await.unwrap();

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

    #[tokio::test]
    async fn test_write_compare_result() {
        let out_path = PathBuf::from("test_write_compare_result.txt");
        let mut out_writer = OutWriter::new(&out_path).await.unwrap();

        // Create some test data
        let class1 = ClassFromSchedule {
            subject_name: "Math".to_string(),
            class_code: "M101".to_string(),
            day: "Monday".to_string(),
            session_start: "08:00".to_string(),
            lecturer_code: vec!["L1".to_string()],
        };
        let class2 = ClassFromSchedule {
            subject_name: "Physics".to_string(),
            class_code: "P101".to_string(),
            day: "Tuesday".to_string(),
            session_start: "09:00".to_string(),
            lecturer_code: vec!["L2".to_string()],
        };
        let added = vec![class1.clone()];
        let changed = vec![(class1.clone(), class2.clone())];
        let deleted = vec![class2.clone()];

        // Write the compare result to the file
        out_writer
            .write_compare_result(&added, &changed, &deleted)
            .await
            .unwrap();

        // Read the file
        let contents = tokio::fs::read_to_string("test_write_compare_result.txt")
            .await
            .unwrap();

        // Assert that the file contains the compare result
        assert!(contents.contains("--- ADDED ---"));
        assert!(contents.contains("Math M101, Monday 08:00, [\"L1\"]"));
        assert!(contents.contains("--- CHANGED ---"));
        assert!(contents.contains("From DB : Math M101, Monday 08:00, [\"L1\"]"));
        assert!(contents.contains("From Excel : Physics P101, Tuesday 09:00, [\"L2\"]"));
        assert!(contents.contains("--- DELETED ---"));
        assert!(contents.contains("Physics P101, Tuesday 09:00, [\"L2\"]"));

        // Clean up
        tokio::fs::remove_file("test_write_compare_result.txt")
            .await
            .unwrap();
    }
}
