use crate::{
    db::repository::class::{Class, ClassFromSchedule},
    DAYS, DAY_OFFSET,
};
use calamine::DataType;

use super::{AsIdParser, AsStringParser, Excel, ScheduleParser, SessionParser};

impl ScheduleParser<Class> for Excel {
    fn get_schedule(&self) -> Vec<Class> {
        let mut list_class: Vec<Class> = Vec::with_capacity(self.range.get_size().1);

        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                let (subject_id, class_code) = match self.get_subject_id_with_code(val) {
                    Some(val) => val,
                    None => continue,
                };
                let lecturers_id = match self.get_lecturer_id(row_idx as u32, col_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                let day = DAYS[row_idx / DAY_OFFSET];
                let session_id = match self.get_session(row_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                let data = Class {
                    matkul_id: subject_id,
                    lecturers_id,
                    day: day.to_string(),
                    code: class_code,
                    session_id,
                };
                list_class.push(data);
            }
        }
        list_class
    }
}

impl ScheduleParser<ClassFromSchedule> for Excel {
    fn get_schedule(&self) -> Vec<ClassFromSchedule> {
        let mut list_class: Vec<ClassFromSchedule> = Vec::with_capacity(self.range.get_size().1);
        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                let (subject_name, class_code) = match self.get_subject_with_code(val) {
                    Some(val) => val,
                    None => continue,
                };
                let lecturers = match self.get_lecturer(row_idx as u32, col_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                let day = DAYS[row_idx / DAY_OFFSET];
                let session_start = match self.get_session(row_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                let data = ClassFromSchedule {
                    subject_name,
                    class_code,
                    lecturer_code: lecturers,
                    day: day.to_string(),
                    session_start,
                };
                list_class.push(data);
            }
        }
        list_class
    }
}
