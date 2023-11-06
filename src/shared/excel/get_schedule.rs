use crate::shared::repo::Class;

use super::{Excel, GetSchedule, Parser, DAYS};

impl GetSchedule for Excel {
    fn get_subject_id_with_code(&self, val: &str) -> Option<(String, String)> {
        let (subject_name, code) = Self::parse_subject_with_code_2(val)?;
        match self.subject_to_id.get(&subject_name.to_lowercase()) {
            Some(val) => Some((val.to_string(), code)),
            None => None,
        }
    }

    fn get_session_id(&self, row_idx: u32) -> Option<i8> {
        let session_name = self.parse_session(row_idx)?;
        match self.session_to_id.get(&session_name) {
            Some(val) => Some(*val),
            None => None,
        }
    }

    fn get_lecturer_ids(&self, row: u32, col: u32) -> Option<Vec<String>> {
        let lecturers = self.parse_lecturer(row, col)?;
        let lecturers_id: Vec<String> = lecturers
            .into_iter()
            .flat_map(|lecture_code| {
                let id = match self.lecturer_to_id.get(lecture_code.trim()) {
                    Some(code) => code,
                    None => self.lecturer_to_id.get("UNK").unwrap(),
                };
                vec![id.to_string()]
            })
            .collect();

        match lecturers_id.is_empty() {
            true => None,
            false => Some(lecturers_id),
        }
    }

    fn get_schedule(&self) -> Vec<Class> {
        let mut list_class: Vec<Class> = Vec::with_capacity(self.range.get_size().1 as usize);

        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                let (subject_id, class_code) = match self.get_subject_id_with_code(&val) {
                    Some(val) => val,
                    None => continue,
                };
                let lecturers_id = match self.get_lecturer_ids(row_idx as u32, col_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                let day = DAYS[row_idx / 14];
                let session_id = match self.get_session_id(row_idx as u32) {
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
        list_class
    }
}
