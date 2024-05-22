use crate::excel::retrieve::Retrieve;
use crate::excel::Excel;
use crate::parser::Parser;

pub trait AsIdParser {
    fn get_subject_id_with_code(&self, val: &str) -> Option<(String, String)>;
    fn get_lecturer_id(&self, row: u32, col: u32) -> Option<Vec<String>>;
}

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
        let unk_id = self
            .lecturer_subjects_session_map
            .lecturers
            .get("UNK")?
            .to_string();
        let lecturers_id: Vec<String> = lecturers
            .into_iter()
            .map(|lecturer_code| {
                self.lecturer_subjects_session_map
                    .lecturers
                    .get(lecturer_code.trim())
                    .unwrap_or(&unk_id)
                    .to_string()
            })
            .collect();
        Some(lecturers_id)
    }
}

#[cfg(test)]
mod tests {

    use calamine::Range;

    use crate::db::repository::LecturerSubjectSessionMap;

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_subject_id_with_code() {
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
