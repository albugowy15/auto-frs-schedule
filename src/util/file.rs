use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::repo::{Class, ClassFromSchedule};

pub struct Writer {
    file: File,
}

impl Writer {
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
        let added_header = format!("--- ADDED ---\n");
        self.write(added_header).await?;
        for class in added {
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

        let changed_header = format!("\n\n--- CHANGED ---\n");
        self.write(changed_header).await?;
        for class in changed {
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

        let deleted_header = format!("\n\n--- DELETED ---\n");
        self.write(deleted_header).await?;
        for class in deleted {
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
        Ok(())
    }
}
