use std::path::PathBuf;

use crate::excel::{find_class::FindClassSchedule, Excel};

pub async fn find_handler(file: &PathBuf, sheet: &str, subject: &str) -> anyhow::Result<()> {
    let excel = Excel::new(file, sheet)?;

    let class_schedule = excel.find_schedule_from_class(subject);
    for schedule in class_schedule.into_iter() {
        println!(
            "class:{}, lec_codes:{:?}, day: {}, session:{}",
            schedule.class, schedule.lecturers_code, schedule.day, schedule.session_start
        );
    }
    Ok(())
}
