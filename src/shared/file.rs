use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::shared::repo::{Class, ClassFromSchedule};

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
