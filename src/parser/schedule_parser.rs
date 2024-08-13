use crate::{
    db::repository::class::{Class, ClassFromSchedule},
    excel::{retrieve::Retrieve, Excel},
    parser::{
        as_id_parser::AsIdParser, as_string_parser::AsStringParser, session_parser::SessionParser,
    },
    DAYS, DAY_OFFSET,
};
use calamine::DataType;

use super::Parser;

pub trait ScheduleParser<T> {
    fn get_schedule(&self) -> Vec<T>;
}
impl ScheduleParser<Class> for Excel {
    fn get_schedule(&self) -> Vec<Class> {
        let mut list_class: Vec<Class> = Vec::with_capacity(self.range.get_size().1);

        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                // start parse subjects and subject code
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                let (subject_id, class_code) = match self.get_subject_id_with_code(val) {
                    Some(val) => val,
                    None => continue,
                };
                // start parse subjects and subject code

                // start parse lecturers
                let lecturers_str = match self.retrieve_class_detail(row_idx as u32, col_idx as u32)
                {
                    Some(val) => val,
                    None => continue,
                };
                let lecturers = match Excel::parse_lecturer(&lecturers_str) {
                    Some(val) => val,
                    None => continue,
                };
                let lecturers_id = self.get_lecturer_id(lecturers);
                // end parse lecturers

                // start parse sessions
                let day = DAYS[row_idx / DAY_OFFSET];
                let session_id = match self.get_session(row_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                // end parse sessions
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
