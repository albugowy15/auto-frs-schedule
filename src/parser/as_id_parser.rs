use crate::excel::Excel;
use crate::parser::Parser;

pub trait AsIdParser {
    fn get_subject_id_with_code(&self, val: &str) -> Option<(String, String)>;
    fn get_lecturer_id(&self, lecturers: Vec<String>) -> Vec<String>;
}

impl AsIdParser for Excel {
    fn get_subject_id_with_code(&self, val: &str) -> Option<(String, String)> {
        let (subject_name, code) = Self::parse_subject_with_code(val)?;
        self.lecturer_subjects_session_map
            .subjects
            .get(&subject_name.to_lowercase())
            .map(|val| (val.to_string(), code))
    }

    fn get_lecturer_id(&self, lecturers: Vec<String>) -> Vec<String> {
        lecturers
            .into_iter()
            .map(|lecturer_code| {
                let lec_code = lecturer_code.trim();
                let err_value = format!("Cannot find lecturer id for {} lecturer code", lec_code);
                self.lecturer_subjects_session_map
                    .lecturers
                    .get(lec_code)
                    .expect(&err_value)
                    .to_string()
            })
            .collect()
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

        let mut lecturers_map = HashMap::new();
        lecturers_map.insert("AB".to_string(), "c12131666h2h2d223".to_string());
        lecturers_map.insert("AM".to_string(), "c6hhd2766d2d2777d".to_string());
        lecturers_map.insert("FB".to_string(), "c61217hdhwd72hd22".to_string());
        lecturers_map.insert("DS".to_string(), "c2hhee7711777hf77".to_string());

        let excel = Excel {
            lecturer_subjects_session_map: LecturerSubjectSessionMap {
                subjects: subject_to_id,
                lecturers: lecturers_map,
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

    #[test]
    fn test_get_lecturer_id() {
        let mut lecturers_map = HashMap::new();
        lecturers_map.insert("AB".to_string(), "c12131666h2h2d223".to_string());
        lecturers_map.insert("AM".to_string(), "c6hhd2766d2d2777d".to_string());
        lecturers_map.insert("FB".to_string(), "c61217hdhwd72hd22".to_string());
        lecturers_map.insert("DS".to_string(), "c2hhee7711777hf77".to_string());

        let excel = Excel {
            lecturer_subjects_session_map: LecturerSubjectSessionMap {
                subjects: HashMap::new(),
                lecturers: lecturers_map,
                sessions: HashMap::new(),
            },
            range: Range::new((0, 0), (100, 100)),
        };
        struct TestCase {
            lecturers: Vec<String>,
            expected: Vec<String>,
        }
        let test_cases = vec![
            TestCase {
                lecturers: vec![String::from("AB"), String::from("FB")],
                expected: vec![
                    String::from("c12131666h2h2d223"),
                    String::from("c61217hdhwd72hd22"),
                ],
            },
            TestCase {
                lecturers: vec![String::from("AM"), String::from("DS")],
                expected: vec![
                    String::from("c6hhd2766d2d2777d"),
                    String::from("c2hhee7711777hf77"),
                ],
            },
        ];
        for test in test_cases.into_iter() {
            assert_eq!(excel.get_lecturer_id(test.lecturers), test.expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_panic_get_lecturer_id() {
        let excel = Excel {
            lecturer_subjects_session_map: LecturerSubjectSessionMap {
                subjects: HashMap::new(),
                lecturers: HashMap::new(),
                sessions: HashMap::new(),
            },
            range: Range::new((0, 0), (100, 100)),
        };
        let lecturers = vec![String::from("AB"), String::from("FB")];
        excel.get_lecturer_id(lecturers);
    }
}
