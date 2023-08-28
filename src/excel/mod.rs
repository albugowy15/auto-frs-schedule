use std::collections::HashMap;

use calamine::{DataType, Range};

use crate::repo::{Class, ClassFromSchedule};

pub mod excel;
pub mod into_map;
pub mod into_str;
pub mod parser;

pub const DAYS: [&str; 5] = ["Senin", "Selasa", "Rabu", "Kamis", "Jum'at"];

pub struct Excel {
    range: Range<DataType>,
    list_subject: HashMap<String, String>,
    list_lecture: HashMap<String, String>,
    list_session: HashMap<String, i8>,
}

pub trait Parser {
    fn parse_subject_with_code(&self, val: &str) -> Option<(String, String)>;
    fn parse_lecturer(&self, row: u32, col: u32) -> Option<Vec<&str>>;
    fn parse_session(&self, row_idx: u32) -> Option<String>;
}

pub trait IntoStr {
    fn subject_with_code_to_str(&self, val: &str) -> Option<(String, String)>;
    fn lecturer_to_str(&self, row: u32, col: u32) -> Option<Vec<String>>;
    fn session_to_str(&self, row_idx: u32) -> Option<String>;
    fn updated_schedule_to_str(&self) -> Vec<ClassFromSchedule>;
}

pub trait IntoMap {
    fn subject_with_code_to_map(&self, val: &str) -> Option<(String, String)>;
    fn lecturer_to_map(&self, row: u32, col: u32) -> Option<Vec<String>>;
    fn session_to_map(&self, row_idx: u32) -> Option<i8>;
    fn parse_excel(&self) -> Vec<Class>;
}
