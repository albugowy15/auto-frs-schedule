use std::path::PathBuf;

use crate::utils::excel::{Excel, FindClassSchedule};

pub async fn find_handler(file: &PathBuf, sheet: &str, subject: &str) -> anyhow::Result<()> {
    println!("Parse class schedule from Excel");
    let excel = Excel::new(file, sheet)?;

    let class_schedule = excel.find_schedule_from_class(subject);
    println!("Found {} schedules for {}", class_schedule.len(), subject);
    for schedule in class_schedule.into_iter() {
        println!(
            "class:{}, lec_codes:{:?}, day: {}, session:{}",
            schedule.class, schedule.lecturers_code, schedule.day, schedule.session_start
        );
    }
    println!("Done");
    Ok(())
}
