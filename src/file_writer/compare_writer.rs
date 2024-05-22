use crate::db::repository::class::ClassFromSchedule;
use crate::file_writer::class_writer::ClassFileWriter;

use super::FileWriter;

pub enum CompareVecResult<'a> {
    DBAndExcel(&'a [(ClassFromSchedule, ClassFromSchedule)]),
    Excel(&'a [ClassFromSchedule]),
}

pub trait CompareFileWriter {
    fn write_change(
        &mut self,
        header: &str,
        result: &CompareVecResult<'_>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn write_compare_result(
        &mut self,
        added: &[ClassFromSchedule],
        changed: &[(ClassFromSchedule, ClassFromSchedule)],
        deleted: &[ClassFromSchedule],
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

impl CompareFileWriter for FileWriter {
    async fn write_change(
        &mut self,
        header: &str,
        result: &CompareVecResult<'_>,
    ) -> anyhow::Result<()> {
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

    async fn write_compare_result(
        &mut self,
        added: &[ClassFromSchedule],
        changed: &[(ClassFromSchedule, ClassFromSchedule)],
        deleted: &[ClassFromSchedule],
    ) -> anyhow::Result<()> {
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

    use crate::{
        db::repository::class::ClassFromSchedule,
        file_writer::{
            compare_writer::{CompareFileWriter, CompareVecResult},
            FileWriter,
        },
    };

    #[tokio::test]
    async fn test_write_change() {
        let out_path = PathBuf::from("test_write_change.txt");
        let mut file_writer = FileWriter::new(&out_path).await.unwrap();

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
        file_writer
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
    async fn test_write_compare_result() {
        let out_path = PathBuf::from("test_write_compare_result.txt");
        let mut file_writer = FileWriter::new(&out_path).await.unwrap();

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
        file_writer
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
