pub mod parser;
pub mod retrieve;
pub mod schedule_parser;
pub mod schedule_parser_with_id;

use std::path::PathBuf;

use anyhow::{Context, Result};
use calamine::{open_workbook, Data, DataType, Range, Reader, Xlsx};

use crate::{
    db::repository::{class::ClassFindSchedule, LecturerSubjectSessionMap},
    DAYS,
};

pub struct Excel {
    range: Range<Data>,
    lecturer_subjects_session_map: LecturerSubjectSessionMap,
}

impl Excel {
    pub fn new(file_path: &PathBuf, sheet_name: &str) -> Result<Self> {
        let mut excel: Xlsx<_> =
            open_workbook(file_path).with_context(|| "Cannot open excel file")?;
        let range = excel.worksheet_range(sheet_name)?;
        Ok(Self {
            range,
            lecturer_subjects_session_map: LecturerSubjectSessionMap::default(),
        })
    }

    pub fn with_repo_data(
        mut self,
        lecturer_subjects_session_map: LecturerSubjectSessionMap,
    ) -> Excel {
        self.lecturer_subjects_session_map = lecturer_subjects_session_map;
        self
    }
}

pub trait Retrieve {
    fn retrieve_class_detail(&self, row_idx: u32, col_idx: u32) -> Option<String>;
    fn retrieve_session(&self, row_idx: u32) -> Option<String>;
}

pub trait Parser {
    fn parse_lecturer(class_detail_str: &str) -> Option<Vec<String>>;
    fn parse_session(session_str: &str) -> Option<String>;
    fn parse_subject_with_code(val: &str) -> Option<(String, String)>;
}

trait AsIdParser {
    fn get_subject_id_with_code(&self, val: &str) -> Option<(String, String)>;
    fn get_lecturer_id(&self, row: u32, col: u32) -> Option<Vec<String>>;
}

trait AsStringParser {
    fn get_subject_with_code(&self, val: &str) -> Option<(String, String)>;
    fn get_lecturer(&self, row: u32, col: u32) -> Option<Vec<String>>;
}

trait SessionParser<T> {
    fn get_session(&self, row_idx: u32) -> Option<T>;
}

pub trait ScheduleParser<T> {
    fn get_schedule(&self) -> Vec<T>;
}

pub trait FindClassSchedule {
    fn find_schedule_from_class(&self, subject_name: &str) -> Vec<ClassFindSchedule>;
}

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
