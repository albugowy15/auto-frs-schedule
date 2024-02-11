use std::path::PathBuf;

use crate::utils::excel::{Excel, FindClassSchedule};

pub async fn find_handler(file: &PathBuf, sheet: &str, subject: &str) {
    log::info!("Parse class schedule from Excel");
    let excel = match Excel::new(file, sheet) {
        Ok(excel) => excel,
        Err(e) => {
            log::error!("Error opening excel file: {}", e);
            return;
        }
    };

    let class_schedule = excel.find_schedule_from_class(subject);
    log::info!("Found {} schedules for {}", class_schedule.len(), subject);
    for schedule in class_schedule.into_iter() {
        log::info!(
            "class:{}, lec_codes:{:?}, day: {}, session:{}",
            schedule.class,
            schedule.lecturers_code,
            schedule.day,
            schedule.session_start
        );
    }
}
