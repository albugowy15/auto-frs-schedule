use crate::db::repository::class::ClassFromSchedule;

use super::{AsStringParser, Excel, Parser, Retrieve, ScheduleParser, SessionParser, DAYS};

impl AsStringParser for Excel {
    fn get_subject_with_code(&self, val: &str) -> Option<(String, String)> {
        let (subject_name, code) = Self::parse_subject_with_code_2(val)?;
        self.lecturer_subjects_session_map
            .subjects
            .get(&subject_name.to_lowercase())
            .map(|_| (subject_name, code))
    }

    fn get_lecturer(&self, row: u32, col: u32) -> Option<Vec<String>> {
        let lecturers_str = self.retrieve_class_detail(row, col)?;
        let lecturers = Excel::parse_lecturer(&lecturers_str)?;
        let lecturers_code: Vec<String> = lecturers
            .into_iter()
            .flat_map(|lecture_code| {
                let code = match self
                    .lecturer_subjects_session_map
                    .lecturers
                    .contains_key(lecture_code.trim())
                {
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
}

impl SessionParser<String> for Excel {
    fn get_session(&self, row_idx: u32) -> Option<String> {
        let session_str = self.retrieve_session(row_idx)?;
        let session_name = Excel::parse_session(&session_str)?;
        self.lecturer_subjects_session_map
            .sessions
            .contains_key(&session_name)
            .then_some(session_name)
    }
}

impl ScheduleParser<ClassFromSchedule> for Excel {
    fn get_schedule(&self) -> Vec<ClassFromSchedule> {
        let mut list_class: Vec<ClassFromSchedule> = Vec::with_capacity(self.range.get_size().1);
        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                let (subject_name, class_code) = match self.get_subject_with_code(val) {
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
        let result = excel.get_subject_with_code("Jaringan Komputer C");
        assert_eq!(
            result,
            Some(("Jaringan Komputer".to_string(), "C".to_string()))
        );

        let result = excel.get_subject_with_code("Physics P101");
        assert_eq!(result, None);
    }
}
