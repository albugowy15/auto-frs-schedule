pub mod parser;
pub mod retrieve;
pub mod schedule_parser;
pub mod schedule_parser_with_id;

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};

pub const DAYS: [&str; 5] = ["Senin", "Selasa", "Rabu", "Kamis", "Jum'at"];

pub struct Excel {
    range: Range<DataType>,
    subject_to_id: HashMap<String, String>,
    lecturer_to_id: HashMap<String, String>,
    session_to_id: HashMap<String, i8>,
}

impl Excel {
    pub fn new(
        file_path: &PathBuf,
        sheet_name: &str,
        subject_to_id: HashMap<String, String>,
        lecturer_to_id: HashMap<String, String>,
        session_to_id: HashMap<String, i8>,
    ) -> Result<Self> {
        let mut excel: Xlsx<_> =
            open_workbook(file_path).with_context(|| "Cannot open excel file")?;
        let range = excel.worksheet_range(sheet_name)?;
        Ok(Self {
            range,
            subject_to_id,
            lecturer_to_id,
            session_to_id,
        })
    }
}

pub trait Retrieve {
    fn retrieve_class_detail(&self, row_idx: u32, col_idx: u32) -> Option<String>;
    fn retrieve_session(&self, row_idx: u32) -> Option<String>;
}

pub trait Parser<'a> {
    fn parse_subject_with_code(val: &str) -> Option<(String, String)>;
    fn parse_lecturer(class_detail_str: &'a str) -> Option<Vec<&'a str>>;
    fn parse_session(session_str: &str) -> Option<String>;
    fn parse_subject_with_code_2(val: &str) -> Option<(String, String)>;
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
