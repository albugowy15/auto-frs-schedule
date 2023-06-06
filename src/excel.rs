use std::collections::HashMap;

/*
Data :
Matkul -> Matkul id
Lecture -> Lecture id
day
code
is Akses -> set false
taken -> set 0
session -> session Id
 */
use calamine::{open_workbook, Error, Reader, Xlsx};

#[allow(dead_code)]
pub struct Class {
    pub matkul_id: String,
    pub lecture_id: String,
    pub day: String,
    pub code: String,
    pub is_akses: bool,
    pub taken: u32,
    pub session_id: u32,
}

pub fn parse_excel(
    list_subject: &HashMap<String, String>,
    list_lecture: &HashMap<String, String>,
    list_session: &HashMap<String, u32>,
) -> Result<Vec<Class>, Error> {
    let path = format!(
        "{}/assets/Jadwal Kuliah Genap 22-23 T.Informatika ITS.xlsx",
        env!("CARGO_MANIFEST_DIR")
    );
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    let range = excel.worksheet_range("Jadwal Kuliah").unwrap()?;

    let mut list_class: Vec<Class> = Vec::new();

    for (row_idx, row) in range.rows().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            if let Some(val) = c.get_string() {
                // get subject
                let subject_name = val.split(" - ").collect::<Vec<&str>>();
                if list_subject.contains_key(subject_name[0]) {
                    let subject_id = list_subject.get(subject_name[0]).unwrap();
                    let lecture_code = range
                        .get_value((row_idx as u32 + 1, col_idx as u32))
                        .unwrap()
                        .get_string()
                        .unwrap()
                        .split("/")
                        .collect::<Vec<&str>>()[2];
                    if lecture_code.len() != 2 {
                        continue;
                    }
                    let lecture_id = match list_lecture.get(lecture_code) {
                        Some(val) => val,
                        None => {
                            println!("No lecture id for {}", lecture_code);
                            continue;
                        }
                    };

                    let day = match row_idx {
                        0..=14 => "Senin",
                        15..=28 => "Selasa",
                        29..=42 => "Rabu",
                        43..=56 => "Kamis",
                        _ => "Jum'at",
                    };
                    // get code
                    let class_code = subject_name[1];

                    // session id
                    let session_name = range
                        .get_value((row_idx as u32, 1))
                        .unwrap()
                        .get_string()
                        .unwrap()
                        .split(" - ")
                        .collect::<Vec<&str>>()[0];
                    let session_id = match list_session.get(session_name) {
                        Some(val) => val,
                        None => {
                            println!("No session id for : {}", session_name);
                            continue;
                        }
                    };

                    let class = Class {
                        matkul_id: subject_id.to_string(),
                        lecture_id: lecture_id.to_string(),
                        day: day.to_string(),
                        code: class_code.to_string(),
                        is_akses: false,
                        taken: 0,
                        session_id: *session_id,
                    };

                    list_class.push(class);
                }

                // get lecture
            }
        }
    }
    println!("len class = {}", list_class.len());

    Ok(list_class)
}
