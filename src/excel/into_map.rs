use crate::repo::Class;

use super::{Excel, IntoMap, Parser, DAYS};

impl IntoMap for Excel {
    fn subject_with_code_to_map(&self, val: &str) -> Option<(String, String)> {
        let (subject_name, code) = self.parse_subject_with_code(val)?;
        match self.list_subject.get(&subject_name.to_lowercase()) {
            Some(val) => Some((val.to_string(), code)),
            None => None,
        }
    }

    fn session_to_map(&self, row_idx: u32) -> Option<i8> {
        let session_name = self.parse_session(row_idx)?;
        match self.list_session.get(&session_name) {
            Some(val) => Some(*val),
            None => None,
        }
    }

    fn lecturer_to_map(&self, row: u32, col: u32) -> Option<Vec<String>> {
        let lecturers = self.parse_lecturer(row, col)?;
        let lecturers_id: Vec<String> = lecturers
            .into_iter()
            .flat_map(|lecture_code| {
                let id = match self.list_lecture.get(lecture_code.trim()) {
                    Some(code) => code,
                    None => self.list_lecture.get("UNK").unwrap(),
                };
                vec![id.to_string()]
            })
            .collect();

        match lecturers_id.is_empty() {
            true => None,
            false => Some(lecturers_id),
        }
    }

    fn parse_excel(&self) -> Vec<Class> {
        let mut list_class: Vec<Class> = Vec::with_capacity(self.range.get_size().1 as usize);

        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                let (subject_id, class_code) = match self.subject_with_code_to_map(&val) {
                    Some(val) => val,
                    None => continue,
                };
                let lecturers_id = match self.lecturer_to_map(row_idx as u32, col_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                let day = DAYS[row_idx / 14];
                let session_id = match self.session_to_map(row_idx as u32) {
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
