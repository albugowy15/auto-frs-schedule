use std::path::PathBuf;

use crate::excel::{find_class::FindClassSchedule, Excel};

pub async fn find_handler(file: &PathBuf, sheet: &str, subject: &str) -> anyhow::Result<()> {
    let excel = Excel::new(file, sheet)?;

    let class_schedule = excel.find_schedule_from_class(subject);
    println!(
        "{0: <60} | {1: <35} | {2: <10} | {3: <10}",
        "class", "lecturer codes", "day", "session"
    );
    for schedule in class_schedule.into_iter() {
        println!(
            "{0: <60} | {1: <35} | {2: <10} | {3: <10}",
            schedule.class,
            format!("{:?}", schedule.lecturers_code),
            schedule.day,
            schedule.session_start
        );
    }
    Ok(())
}
