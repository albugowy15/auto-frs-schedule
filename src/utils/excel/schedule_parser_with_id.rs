use crate::db::repository::class_repository::Class;

use super::{AsIdParser, Excel, Parser, ScheduleParser, SessionParser, DAYS};

impl AsIdParser for Excel {
    fn get_subject_id_with_code(&self, val: &str) -> Option<(String, String)> {
        let (subject_name, code) = Self::parse_subject_with_code_2(val)?;
        self.subject_to_id
            .get(&subject_name.to_lowercase())
            .map(|val| (val.to_string(), code))
    }

    fn get_lecturer_id(&self, row: u32, col: u32) -> Option<Vec<String>> {
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
}

impl SessionParser<i8> for Excel {
    fn get_session(&self, row_idx: u32) -> Option<i8> {
        let session_name = self.parse_session(row_idx)?;
        self.session_to_id.get(&session_name).copied()
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

    use crate::utils::excel::{AsIdParser, Excel};

    #[test]
    fn test_get_subject_with_code() {
        // Create a parser
        let mut subject_to_id = HashMap::new();
        subject_to_id.insert(
            "jaringan komputer".to_string(),
            "c6hhfe7737483833".to_string(),
        );

        let excel = Excel {
            subject_to_id,
            lecturer_to_id: HashMap::new(),
            session_to_id: HashMap::new(),
            range: Range::new((0, 0), (100, 100)),
        };

        // Test the get_subject_with_code method
        let result = excel.get_subject_id_with_code("Jaringan Komputer C");
        assert_eq!(
            result,
            Some(("c6hhfe7737483833".to_string(), "C".to_string()))
        );

        let result = excel.get_subject_id_with_code("Physics P101");
        assert_eq!(result, None);
    }
}
