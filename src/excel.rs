use crate::repo::Class;
use anyhow::{Context, Result};
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::{collections::HashMap, path::PathBuf};

pub struct Excel {
    range: Range<DataType>,
}

impl Excel {
    pub fn new(file_path: &PathBuf, sheet_name: &str) -> Result<Self> {
        let mut excel: Xlsx<_> =
            open_workbook(file_path).with_context(|| "Cannot open excel file")?;
        let range = excel
            .worksheet_range(sheet_name)
            .context("Error opening sheet, make sure sheet name is exists")?
            .with_context(|| format!("Could not read excel range from sheet {}", sheet_name))?;
        Ok(Self { range })
    }
    fn parse_subject_class(
        val: &str,
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
    ) -> Option<Vec<String>> {
        let lecturer = self
            .range
            .get_value((row + 1, col))?
            .get_string()?
            .split("/")
            .collect::<Vec<_>>()[2];
        let lecturers_id: Vec<_> = lecturer
            .split("-")
            .flat_map(|lecture_code| lecturer_map.get(lecture_code.trim()))
            .map(String::from)
            .collect();
        if lecturers_id.is_empty() {
            return None;
        } else {
            Some(lecturers_id)
        }
    }
    fn parse_session(&self, row_idx: u32, session_map: &HashMap<String, i8>) -> Option<i8> {
        let session_name = self
            .range
            .get_value((row_idx, 1))?
            .get_string()?
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
        list_session: &HashMap<String, i8>,
    ) -> Result<Vec<Class>> {
        let mut list_class: Vec<Class> = Vec::with_capacity(self.range.get_size().1 as usize);

        let days = ["Senin", "Selasa", "Rabu", "Kamis", "Jum'at"];

        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };

                let (subject_id, class_code) = match Self::parse_subject_class(&val, &list_subject)
                {
                    Some(val) => val,
                    None => continue,
                };

                let lecturers_id =
                    match self.parse_lecturer(row_idx as u32, col_idx as u32, &list_lecture) {
                        Some(val) => val,
                        None => continue,
                    };

                let day = days[row_idx / 15];

                let session_id = match self.parse_session(row_idx as u32, &list_session) {
                    Some(val) => val,
                    None => continue,
                };

                list_class.push(Class {
                    matkul_id: subject_id,
                    lecturers_id,
                    day: day.to_string(),
                    code: class_code,
                    session_id,
                });
            }
        }

        Ok(list_class)
    }
}
