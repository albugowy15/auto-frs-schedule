use std::collections::HashMap;

use calamine::{DataType, Range};

use crate::repo::{Class, ClassFromSchedule};

pub mod excel;
pub mod get_schedule;
pub mod get_schedule_update;
pub mod parser;

pub const DAYS: [&str; 5] = ["Senin", "Selasa", "Rabu", "Kamis", "Jum'at"];

pub struct Excel {
    range: Range<DataType>,
    subject_to_id: HashMap<String, String>,
    lecturer_to_id: HashMap<String, String>,
    session_to_id: HashMap<String, i8>,
}

pub trait Parser {
    fn parse_subject_with_code(val: &str) -> Option<(String, String)>;
    fn parse_lecturer(&self, row: u32, col: u32) -> Option<Vec<&str>>;
    fn parse_session(&self, row_idx: u32) -> Option<String>;
    fn parse_subject_with_code_2(val: &str) -> Option<(String, String)>;
}

pub trait GetScheduleUpdate {
    fn get_subject_with_code(&self, val: &str) -> Option<(String, String)>;
    fn get_lecturer(&self, row: u32, col: u32) -> Option<Vec<String>>;
    fn get_session(&self, row_idx: u32) -> Option<String>;
    fn get_updated_schedule(&self) -> Vec<ClassFromSchedule>;
}

pub trait GetSchedule {
    fn get_subject_id_with_code(&self, val: &str) -> Option<(String, String)>;
    fn get_lecturer_ids(&self, row: u32, col: u32) -> Option<Vec<String>>;
    fn get_session_id(&self, row_idx: u32) -> Option<i8>;
    fn get_schedule(&self) -> Vec<Class>;
}
