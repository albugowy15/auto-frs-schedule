use crate::db::repository::class::ClassFromSchedule;
use crate::file_writer::FileWriter;

pub trait ClassFileWriter {
    fn write_class_info(
        &mut self,
        prefix: &str,
        class: &ClassFromSchedule,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}

impl ClassFileWriter for FileWriter {
    async fn write_class_info(
        &mut self,
        prefix: &str,
        class: &ClassFromSchedule,
    ) -> anyhow::Result<()> {
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
}
