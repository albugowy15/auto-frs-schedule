use std::collections::HashMap;

use anyhow::Result;

use crate::repo::Class;

use super::{Excel, IntoMap, DAYS};

impl IntoMap for Excel {
    fn subject_class_to_map(
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
        match subject_map.get(&subject_name.to_lowercase()) {
            Some(val) => Some((val.to_string(), code)),
            None => None,
        }
    }

    fn session_to_map(&self, row_idx: u32, session_map: &HashMap<String, i8>) -> Option<i8> {
        let session_name = self
            .range
            .get_value((row_idx, 1))?
            .get_string()?
            .split(" - ")
            .collect::<Vec<&str>>()[0];
        match session_map.get(session_name) {
            Some(val) => Some(*val),
            None => None,
        }
    }

    fn lecturer_to_map(
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
                let id = match lecturer_map.get(lecture_code.trim()) {
                    Some(code) => code,
                    None => lecturer_map.get("UNK").unwrap(),
                };
                vec![id.to_string()]
            })
            .collect();
        match lecturers_id.is_empty() {
            true => None,
            false => Some(lecturers_id),
        }
    }

    fn parse_excel(
        &self,
        list_subject: &HashMap<String, String>,
        list_lecture: &HashMap<String, String>,
        list_session: &HashMap<String, i8>,
    ) -> Result<Vec<Class>> {
        let mut list_class: Vec<Class> = Vec::with_capacity(self.range.get_size().1 as usize);

        for (row_idx, row) in self.range.rows().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
                let val = match c.get_string() {
                    Some(val) => val,
                    None => continue,
                };

                let (subject_id, class_code) =
                    match Excel::subject_class_to_map(&val, &list_subject) {
                        Some(val) => val,
                        None => continue,
                    };

                let lecturers_id =
                    match self.lecturer_to_map(row_idx as u32, col_idx as u32, &list_lecture) {
                        Some(val) => val,
                        None => continue,
                    };

                let day = DAYS[row_idx / 14];

                let session_id = match self.session_to_map(row_idx as u32, &list_session) {
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

        Ok(list_class)
    }
}
