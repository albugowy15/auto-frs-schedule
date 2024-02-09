use crate::{db::repository::class::Class, DAYS};
use calamine::DataType;

use super::{AsIdParser, Excel, Parser, Retrieve, ScheduleParser, SessionParser};

impl AsIdParser for Excel {
    fn get_subject_id_with_code(&self, val: &str) -> Option<(String, String)> {
        let (subject_name, code) = Self::parse_subject_with_code(val)?;
        self.lecturer_subjects_session_map
            .subjects
            .get(&subject_name.to_lowercase())
            .map(|val| (val.to_string(), code))
    }

    fn get_lecturer_id(&self, row: u32, col: u32) -> Option<Vec<String>> {
        let lecturers_str = self.retrieve_class_detail(row, col)?;
        let lecturers = Excel::parse_lecturer(&lecturers_str)?;
        let lecturers_id: Vec<String> = lecturers
            .into_iter()
            .map(|lecture_code| {
                self.lecturer_subjects_session_map
                    .lecturers
                    .get(lecture_code.trim())
                    .unwrap_or(&"UNK".to_string())
                    .to_string()
            })
            .collect();

        lecturers_id.into_iter().next().map(|first| vec![first])
    }
}

impl SessionParser<i8> for Excel {
    fn get_session(&self, row_idx: u32) -> Option<i8> {
        let session_str = self.retrieve_session(row_idx)?;
        let session_name = Excel::parse_session(&session_str)?;
        self.lecturer_subjects_session_map
            .sessions
            .get(&session_name)
            .cloned()
    }
}

impl ScheduleParser<Class> for Excel {
    fn get_schedule(&self) -> Vec<Class> {
        let mut list_class: Vec<Class> = Vec::with_capacity(self.range.get_size().1);

        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                let (subject_id, class_code) = match self.get_subject_id_with_code(val) {
                    Some(val) => val,
                    None => continue,
                };
                let lecturers_id = match self.get_lecturer_id(row_idx as u32, col_idx as u32) {
                    Some(val) => val,
                    None => continue,
                };
                let day = DAYS[row_idx / 14];
                let session_id = match self.get_session(row_idx as u32) {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use calamine::Range;

    use crate::db::repository::LecturerSubjectSessionMap;

    use super::*;

    #[test]
    fn test_get_subject_with_code() {
        // Create a parser
        let mut subject_to_id = HashMap::new();
        subject_to_id.insert(
            "jaringan komputer".to_string(),
            "c6hhfe7737483833".to_string(),
        );

        let excel = Excel {
            lecturer_subjects_session_map: LecturerSubjectSessionMap {
                subjects: subject_to_id,
                lecturers: HashMap::new(),
                sessions: HashMap::new(),
            },
            range: Range::new((0, 0), (100, 100)),
        };

        // Test the get_subject_with_code method
        let result = excel.get_subject_id_with_code("Jaringan Komputer - C");
        assert_eq!(
            result,
            Some(("c6hhfe7737483833".to_string(), "C".to_string()))
        );

        let result = excel.get_subject_id_with_code("Physics - P101");
        assert_eq!(result, None);
    }
}
