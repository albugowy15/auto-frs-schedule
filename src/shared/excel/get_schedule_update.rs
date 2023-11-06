use crate::shared::repo::ClassFromSchedule;

use super::{Excel, GetScheduleUpdate, Parser, DAYS};

impl GetScheduleUpdate for Excel {
    fn get_subject_with_code(&self, val: &str) -> Option<(String, String)> {
        let (subject_name, code) = Self::parse_subject_with_code_2(val)?;
        match self
            .subject_to_id
            .contains_key(&subject_name.to_lowercase())
        {
            true => Some((subject_name, code)),
            false => None,
        }
    }

    fn get_lecturer(&self, row: u32, col: u32) -> Option<Vec<String>> {
        let lecturers = self.parse_lecturer(row, col)?;
        let lecturers_code: Vec<String> = lecturers
            .into_iter()
            .flat_map(|lecture_code| {
                let code = match self.lecturer_to_id.contains_key(lecture_code.trim()) {
                    true => lecture_code.trim().to_string(),
                    false => "UNK".to_string(),
                };
                vec![code.to_string()]
            })
            .collect();

        match lecturers_code.is_empty() {
            true => None,
            false => Some(lecturers_code),
        }
    }

    fn get_session(&self, row_idx: u32) -> Option<String> {
        let session_name = self.parse_session(row_idx)?;
        match self.session_to_id.contains_key(&session_name) {
            true => Some(session_name),
            false => None,
        }
    }

    fn get_updated_schedule(&self) -> Vec<ClassFromSchedule> {
        let mut list_class: Vec<ClassFromSchedule> =
            Vec::with_capacity(self.range.get_size().1 as usize);
        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                let (subject_name, class_code) = match self.get_subject_with_code(&val) {
                    Some(val) => val,
                    None => continue,
                };
                let lecturers = match self.get_lecturer(row_idx as u32, col_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                let day = DAYS[row_idx / 14];
                let session_start = match self.get_session(row_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                list_class.push(ClassFromSchedule {
                    subject_name,
                    class_code,
                    lecturer_code: lecturers,
                    day: day.to_string(),
                    session_start,
                })
            }
        }
        list_class
    }
}
