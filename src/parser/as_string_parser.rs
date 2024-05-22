use crate::excel::retrieve::Retrieve;
use crate::excel::Excel;
use crate::parser::Parser;

pub trait AsStringParser {
    fn get_subject_with_code(&self, val: &str) -> Option<(String, String)>;
    fn get_lecturer(&self, row: u32, col: u32) -> Option<Vec<String>>;
}

impl AsStringParser for Excel {
    fn get_subject_with_code(&self, val: &str) -> Option<(String, String)> {
        let (subject_name, code) = Self::parse_subject_with_code(val)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::repository::LecturerSubjectSessionMap;
    use calamine::Range;
    use std::collections::HashMap;

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
        let result = excel.get_subject_with_code("Jaringan Komputer - C");
        assert_eq!(
            result,
            Some(("Jaringan Komputer".to_string(), "C".to_string()))
        );

        let result = excel.get_subject_with_code("Physics - P101");
        assert_eq!(result, None);
    }
}
