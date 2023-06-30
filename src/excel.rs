use crate::repo::Class;
use calamine::{open_workbook, DataType, Error, Range, Reader, Xlsx};
use std::collections::HashMap;

pub struct Excel {
    range: Range<DataType>,
}

impl Excel {
    pub fn new(file_path: &String, sheet_name: &String) -> Result<Self, Error> {
        let mut excel: Xlsx<_> = open_workbook(file_path)?;
        let range = excel.worksheet_range(sheet_name).unwrap()?;
        Ok(Self { range })
    }
    fn parse_subject_class(
        val: &&str,
        subject_map: &HashMap<String, String>,
    ) -> Option<(String, String)> {
        let subject_name = val.split("-").collect::<Vec<&str>>();
        if subject_name.len() < 2 {
            return None;
        }
        let (subject_valid, class_code) =
            if subject_name[0].contains("IUP") && subject_name[1].contains("akselerasi") {
                (
                    subject_name[0].split("IUP").collect::<Vec<&str>>()[0].trim(),
                    "IUP akselerasi",
                )
            } else {
                (subject_name[0].trim(), subject_name[1].trim())
            };
        let subject_id = match subject_map.get(subject_valid) {
            Some(val) => val,
            None => {
                return None;
            }
        };
        Some((subject_id.to_string(), class_code.to_string()))
    }
    fn parse_lecturer(
        &self,
        row: u32,
        col: u32,
        lecturer_map: &HashMap<String, String>,
    ) -> Option<String> {
        let lecturer = self
            .range
            .get_value((row + 1, col))
            .unwrap()
            .get_string()
            .unwrap()
            .split("/")
            .collect::<Vec<&str>>()[2];
        let lecturer_code = if lecturer.len() > 2 {
            lecturer.split("-").collect::<Vec<&str>>()[0].trim()
        } else {
            lecturer
        };
        let lecture_id = match lecturer_map.get(lecturer_code) {
            Some(val) => val,
            None => {
                return None;
            }
        };
        Some(lecture_id.to_string())
    }
    fn parse_session(&self, row_idx: u32, session_map: &HashMap<String, u32>) -> Option<u32> {
        let session_name = self
            .range
            .get_value((row_idx, 1))
            .unwrap()
            .get_string()
            .unwrap()
            .split(" - ")
            .collect::<Vec<&str>>()[0];
        let session_id = match session_map.get(session_name) {
            Some(val) => *val,
            None => {
                return None;
            }
        };
        Some(session_id)
    }
    pub fn parse_excel(
        &self,
        list_subject: &HashMap<String, String>,
        list_lecture: &HashMap<String, String>,
        list_session: &HashMap<String, u32>,
    ) -> Result<Vec<Class>, Error> {
        let mut list_class: Vec<Class> = Vec::new();
        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => {
                        continue;
                    }
                };
                let (subject_id, class_code) = match Self::parse_subject_class(&val, &list_subject)
                {
                    Some(val) => val,
                    None => {
                        continue;
                    }
                };
                let lecturer_id =
                    match self.parse_lecturer(row_idx as u32, col_idx as u32, &list_lecture) {
                        Some(val) => val,
                        None => {
                            continue;
                        }
                    };
                let day = match row_idx {
                    0..=14 => "Senin",
                    15..=28 => "Selasa",
                    29..=42 => "Rabu",
                    43..=56 => "Kamis",
                    _ => "Jum'at",
                };
                let session_id = match self.parse_session(row_idx as u32, &list_session) {
                    Some(val) => val,
                    None => {
                        continue;
                    }
                };
                list_class.push(Class {
                    matkul_id: subject_id.to_string(),
                    lecture_id: lecturer_id.to_string(),
                    day: day.to_string(),
                    code: class_code.to_string(),
                    session_id,
                });
            }
        }
        Ok(list_class)
    }
}
