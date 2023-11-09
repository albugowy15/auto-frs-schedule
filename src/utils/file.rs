use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::db::repository::class_repository::{Class, ClassFromSchedule};

enum CompareVecResult<'a> {
    DBAndExcel(&'a Vec<(ClassFromSchedule, ClassFromSchedule)>),
    Excel(&'a Vec<ClassFromSchedule>),
}

pub struct OutWriter {
    file: File,
}

impl OutWriter {
    pub async fn new(out_path: &PathBuf) -> Result<Self> {
        let outfile = File::create(out_path.as_path()).await?;
        Ok(Self { file: outfile })
    }

    async fn write(&mut self, query: String) -> Result<()> {
        self.file
            .write_all(query.as_bytes())
            .await
            .with_context(|| format!("Error writing to file: {}", query))?;
        Ok(())
    }

    async fn sync_all(&mut self) -> Result<()> {
        self.file.sync_all().await?;
        Ok(())
    }

    async fn write_change(&mut self, header: &str, result: &CompareVecResult<'_>) -> Result<()> {
        let header_section = format!("--- {} ---\n", header);
        self.write(header_section).await?;
        match result {
            CompareVecResult::DBAndExcel(data) => {
                for class in data.into_iter() {
                    let query = format!(
                        "From DB : {} {}, {} {}, {:?}\n",
                        class.0.subject_name,
                        class.0.class_code,
                        class.0.day,
                        class.0.session_start,
                        class.0.lecturer_code
                    );
                    self.write(query).await?;
                    let query = format!(
                        "From Excel : {} {}, {} {}, {:?}\n",
                        class.1.subject_name,
                        class.1.class_code,
                        class.1.day,
                        class.1.session_start,
                        class.1.lecturer_code
                    );
                    self.write(query).await?;
                }
            }
            CompareVecResult::Excel(data) => {
                for class in data.into_iter() {
                    let query = format!(
                        "{} {}, {} {}, {:?}\n",
                        class.subject_name,
                        class.class_code,
                        class.day,
                        class.session_start,
                        class.lecturer_code
                    );
                    self.write(query).await?;
                }
            }
        };

        Ok(())
    }

    #[allow(deprecated)]
    pub async fn write_output(&mut self, list_class: &Vec<Class>) -> Result<()> {
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
        self.write_change("ADDED", &CompareVecResult::Excel(&added))
            .await?;
        self.write_change("CHANGED", &CompareVecResult::DBAndExcel(&changed))
            .await?;
        self.write_change("DELETED", &CompareVecResult::Excel(&deleted))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::db::repository::class_repository::Class;
    use crate::utils::file::ClassFromSchedule;
    use crate::utils::file::CompareVecResult;
    use crate::utils::file::OutWriter;
    use std::path::PathBuf;

    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_write() {
        // Create a temporary file for testing
        let mut temp_file = tokio::fs::File::create("test_write.txt").await.unwrap();

        // Write to the file
        let query = "SELECT * FROM users;".to_string();
        temp_file.write(query.as_bytes()).await.unwrap();

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
