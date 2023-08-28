use std::collections::HashMap;

use crate::repo::ClassFromSchedule;

use super::{Excel, IntoStr, DAYS};

impl IntoStr for Excel {
    fn subject_class_to_str(
        val: &str,
        subject_map: &HashMap<String, String>,
    ) -> Option<(String, String)> {
        let splitted = val.split("-").collect::<Vec<&str>>();
        let subject_name: String;
        let code: String;
        if splitted.len() < 2 {
            let split_space = val.split_ascii_whitespace().collect::<Vec<&str>>();
            let last_str = split_space.last()?.trim();
            if last_str.len() == 1 && last_str <= "L" {
                subject_name = split_space[0..(split_space.len() - 1)].join(" ");
                code = last_str.to_string()
            } else {
                subject_name = split_space.join(" ");
                code = "-".to_owned();
            }
        } else {
            let last_split = splitted.last()?.trim();
            if last_split.contains("EN") {
                let split_space = splitted[0].split_ascii_whitespace().collect::<Vec<&str>>();
                subject_name = split_space[0..(split_space.len() - 1)].join(" ");
                code = format!("{} - {}", split_space.last()?, "EN");
            } else {
                subject_name = splitted[0].trim().to_owned();
                code = splitted[1].trim().to_owned();
            }
        }
        match subject_map.contains_key(&subject_name.to_lowercase()) {
            true => Some((subject_name, code)),
            false => None,
        }
    }

    fn lecturer_to_str(
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
            .flat_map(|lecture_code| {
                let code = match lecturer_map.contains_key(lecture_code.trim()) {
                    true => lecture_code.trim().to_string(),
                    false => "UNK".to_string(),
                };
                vec![code.to_string()]
            })
            .collect();
        match lecturers_id.is_empty() {
            true => None,
            false => Some(lecturers_id),
        }
    }

    fn session_to_str(&self, row_idx: u32, session_map: &HashMap<String, i8>) -> Option<String> {
        let session_name = self
            .range
            .get_value((row_idx, 1))?
            .get_string()?
            .split(" - ")
            .collect::<Vec<&str>>()[0]
            .to_string();
        match session_map.contains_key(&session_name) {
            true => Some(session_name),
            false => None,
        }
    }

    fn updated_schedule_to_str(
        &self,
        list_subject: &HashMap<String, String>,
        list_lecture: &HashMap<String, String>,
        list_session: &HashMap<String, i8>,
    ) -> Vec<ClassFromSchedule> {
        // TODO: parse updated schedule
        // expected data to be returned: ClassFromSchedule

        /*
           parsing steps:
           1. parse subject name and code
           2. parse lecturer code
           3. parse day
           4. parse session start
        */
        let mut list_class: Vec<ClassFromSchedule> = Vec::new();
        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };
                let (subject_name, class_code) =
                    match Excel::subject_class_to_str(&val, &list_subject) {
                        Some(val) => val,
                        None => continue,
                    };
                let lecturers =
                    match self.lecturer_to_str(row_idx as u32, col_idx as u32, &list_lecture) {
                        Some(val) => val,
                        None => continue,
                    };
                let day = DAYS[row_idx / 14];
                let session_start = match self.session_to_str(row_idx as u32, &list_session) {
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
