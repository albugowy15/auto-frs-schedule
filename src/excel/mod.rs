use std::collections::HashMap;

use anyhow::Result;
use calamine::{DataType, Range};

use crate::repo::{Class, ClassFromSchedule};

pub mod excel;
pub mod into_map;
pub mod into_str;

pub const DAYS: [&str; 5] = ["Senin", "Selasa", "Rabu", "Kamis", "Jum'at"];

pub struct Excel {
    range: Range<DataType>,
}

pub trait IntoStr {
    fn subject_class_to_str(
        val: &str,
        subject_map: &HashMap<String, String>,
    ) -> Option<(String, String)>;
    fn lecturer_to_str(
        &self,
        row: u32,
        col: u32,
        lecturer_map: &HashMap<String, String>,
    ) -> Option<Vec<String>>;
    fn session_to_str(&self, row_idx: u32, session_map: &HashMap<String, i8>) -> Option<String>;
    fn updated_schedule_to_str(
        &self,
        list_subject: &HashMap<String, String>,
        list_lecture: &HashMap<String, String>,
        list_session: &HashMap<String, i8>,
    ) -> Vec<ClassFromSchedule>;
}

pub trait IntoMap {
    fn subject_class_to_map(
        val: &str,
        subject_map: &HashMap<String, String>,
    ) -> Option<(String, String)>;
    fn lecturer_to_map(
        &self,
        row: u32,
        col: u32,
        lecturer_map: &HashMap<String, String>,
    ) -> Option<Vec<String>>;
    fn session_to_map(&self, row_idx: u32, session_map: &HashMap<String, i8>) -> Option<i8>;
    fn parse_excel(
        &self,
        list_subject: &HashMap<String, String>,
        list_lecture: &HashMap<String, String>,
        list_session: &HashMap<String, i8>,
    ) -> Result<Vec<Class>>;
}
