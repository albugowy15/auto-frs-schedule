use calamine::DataType;

use crate::{db::repository::class::ClassFindSchedule, DAYS};

use super::{Excel, FindClassSchedule, Parser, Retrieve};

impl FindClassSchedule for Excel {
    fn find_schedule_from_class(&self, subject_name: &str) -> Vec<ClassFindSchedule> {
        let mut schedules: Vec<ClassFindSchedule> = Vec::with_capacity(self.range.get_size().1);
        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                if !val.contains(subject_name) {
                    continue;
                }

                let lecturers_str = match self.retrieve_class_detail(row_idx as u32, col_idx as u32)
                {
                    Some(lecs) => lecs,
                    None => continue,
                };
                let lecturers = match Excel::parse_lecturer(&lecturers_str) {
                    Some(lecs) => lecs,
                    None => continue,
                };
                let day = DAYS[row_idx / 14];

                let session_str = match self.retrieve_session(row_idx as u32) {
                    Some(session) => session,
                    None => continue,
                };
                let session_name = match Excel::parse_session(&session_str) {
                    Some(session) => session,
                    None => continue,
                };
                let data = ClassFindSchedule {
                    class: val.to_string(),
                    lecturers_code: lecturers,
                    day: day.to_string(),
                    session_start: session_name,
                };
                schedules.push(data);
            }
        }
        schedules
    }
}
