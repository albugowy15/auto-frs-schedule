use std::cmp;

use super::{Excel, Parser};

impl<'a> Parser<'a> for Excel {
    fn parse_lecturer(class_detail_str: &'a str) -> Option<Vec<&'a str>> {
        let lecturers: Vec<&str> = class_detail_str
            .split('/')
            .nth(2)?
            .split('-')
            .map(|lec| lec.trim())
            .collect();
        if lecturers.is_empty() {
            None
        } else {
            Some(lecturers)
        }
    }

    fn parse_session(session_str: &str) -> Option<String> {
        session_str.split(" - ").next().map(|s| s.to_string())
    }

    fn parse_subject_with_code(val: &str) -> Option<(String, String)> {
        let elem = val.split('-').map(|val| val.trim()).collect::<Vec<_>>();

        let class_name = elem.first()?;
        match elem.len().cmp(&2) {
            cmp::Ordering::Equal => {
                let code = elem.last()?;
                Some((class_name.to_string(), code.to_string()))
            }

            cmp::Ordering::Greater => {
                let code = elem[1..].join(" ");
                Some((class_name.to_string(), code.to_string()))
            }
            _ => None,
        }
    }

    // fn parse_subject_with_code(val: &str) -> Option<(String, String)> {
    //     // Different parse method for each kind of class code
    //     // CASE 1: (EN) + IUP
    //     if val.contains("(EN) + IUP") {
    //         return Some((
    //             val.split("(EN) + IUP").next()?.trim().to_string(),
    //             "(EN) + IUP".to_string(),
    //         ));
    //     }
    //
    //     // CASE 2: IUP
    //     if val.contains("IUP") {
    //         let subject = val.split("IUP").next()?.trim().split('-').next()?.trim();
    //         // let subject = splitted[0..splitted.len() - 1].join(" ");
    //         return Some((subject.to_string(), "IUP".to_string()));
    //     }
    //
    //     // CASE 3: EN
    //     if val.contains("EN") {
    //         let splitted: Vec<&str> = val
    //             .split("EN")
    //             .next()?
    //             .trim()
    //             .split('-')
    //             .next()?
    //             .split_ascii_whitespace()
    //             .collect();
    //         let code = splitted.last()?.trim();
    //         let subject = splitted[0..splitted.len() - 1].join(" ");
    //         return Some((subject, format!("{} - EN", code)));
    //     }
    //
    //     // CASE 4: - RKA, - RPL
    //     if val.contains('-') {
    //         let mut splitted = val.split('-').map(|x| x.trim());
    //         return Some((splitted.next()?.to_string(), splitted.next()?.to_string()));
    //     }
    //
    //     // CASE 5: Unique Class, examples : Realitas X
    //     if val.contains("Realitas X") {
    //         return Some((val.trim().to_string(), "-".to_string()));
    //     }
    //
    //     // CASE 6: Basic Class, examples: Jaringan Komputer A
    //     let splitted: Vec<&str> = val.split_ascii_whitespace().collect();
    //     if let Some(last_elm) = splitted.last() {
    //         if last_elm.len() == 1 {
    //             let code = last_elm;
    //             let subject = splitted[0..(splitted.len() - 1)].join(" ");
    //             return Some((subject, code.to_string()));
    //         } else {
    //             return Some((splitted.join(" "), "-".to_string()));
    //         }
    //     }
    //     None
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_subject_with_code() {
        struct TestCase {
            class: String,
            subject_name: String,
            subject_code: String,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                class: "Pemrograman Perangkat Bergerak - C".to_string(),
                subject_name: "Pemrograman Perangkat Bergerak".to_string(),
                subject_code: "C".to_string(),
            },
            TestCase {
                class: "Perancangan Perangkat Lunak - IUP".to_string(),
                subject_name: "Perancangan Perangkat Lunak".to_string(),
                subject_code: "IUP".to_string(),
            },
            TestCase {
                class: "Etika Profesi - REG - IUP".to_string(),
                subject_name: "Etika Profesi".to_string(),
                subject_code: "REG IUP".to_string(),
            },
            TestCase {
                class: "Sistem Basis Data - T".to_string(),
                subject_name: "Sistem Basis Data".to_string(),
                subject_code: "T".to_string(),
            },
        ];

        test_cases.into_iter().for_each(|case| {
            assert_eq!(
                Excel::parse_subject_with_code(&case.class),
                Some((case.subject_name, case.subject_code))
            );
        });
    }

    #[test]
    fn test_parse_invalid_subject_with_code() {
        let test_cases: Vec<String> = vec![
            "PANCASILA".to_string(),
            "Kebudayaan dan Kebangsaan".to_string(),
        ];

        test_cases
            .into_iter()
            .for_each(|test| assert_eq!(Excel::parse_subject_with_code(&test), None))
    }
    //#[test]
    // fn test_parse_subject_with_code_2() {
    //     struct TestCase {
    //         class: String,
    //         subject_name: String,
    //         subject_code: String,
    //     }
    //     let test_cases: Vec<TestCase> = vec![
    //         TestCase {
    //             class: "Interaksi Manusia Komputer (EN) + IUP".to_string(),
    //             subject_code: "(EN) + IUP".to_string(),
    //             subject_name: "Interaksi Manusia Komputer".to_string(),
    //         },
    //         TestCase {
    //             class: "Interaksi Manusia Komputer D - EN".to_string(),
    //             subject_code: "D - EN".to_string(),
    //             subject_name: "Interaksi Manusia Komputer".to_string(),
    //         },
    //         TestCase {
    //             class: "Interaksi Manusia Komputer D-EN".to_string(),
    //             subject_code: "D - EN".to_string(),
    //             subject_name: "Interaksi Manusia Komputer".to_string(),
    //         },
    //         TestCase {
    //             class: "Jaringan Komputer - IUP".to_string(),
    //             subject_code: "IUP".to_string(),
    //             subject_name: "Jaringan Komputer".to_string(),
    //         },
    //         TestCase {
    //             class: "Jaringan Komputer-IUP".to_string(),
    //             subject_code: "IUP".to_string(),
    //             subject_name: "Jaringan Komputer".to_string(),
    //         },
    //         TestCase {
    //             class: "Interaksi Manusia Komputer - RKA".to_string(),
    //             subject_code: "RKA".to_string(),
    //             subject_name: "Interaksi Manusia Komputer".to_string(),
    //         },
    //         TestCase {
    //             class: "Interaksi Manusia Komputer-RPL".to_string(),
    //             subject_code: "RPL".to_string(),
    //             subject_name: "Interaksi Manusia Komputer".to_string(),
    //         },
    //         TestCase {
    //             class: "Game Cerdas".to_string(),
    //             subject_code: "-".to_string(),
    //             subject_name: "Game Cerdas".to_string(),
    //         },
    //         TestCase {
    //             class: "Realitas X".to_string(),
    //             subject_code: "-".to_string(),
    //             subject_name: "Realitas X".to_string(),
    //         },
    //         TestCase {
    //             class: "Jaringan Komputer A".to_string(),
    //             subject_code: "A".to_string(),
    //             subject_name: "Jaringan Komputer".to_string(),
    //         },
    //     ];
    //     test_cases.into_iter().for_each(|case| {
    //         assert_eq!(
    //             Excel::parse_subject_with_code(&case.class),
    //             Some((case.subject_name, case.subject_code))
    //         );
    //     });
    // }
    #[test]
    fn test_parse_lecturer_from_class_detail() {
        struct TestCase<'a> {
            class_detail: String,
            lecturers: Vec<&'a str>,
        }
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                class_detail: String::from("2 sks/ Sem 7 / AM - WN"),
                lecturers: vec!["AM", "WN"],
            },
            TestCase {
                class_detail: String::from("4 sks / Sem 2 / CF"),
                lecturers: vec!["CF"],
            },
            TestCase {
                class_detail: String::from("3 sks / Sem 1 / DH"),
                lecturers: vec!["DH"],
            },
            TestCase {
                class_detail: String::from("3 sks / Sem 1 / DO"),
                lecturers: vec!["DO"],
            },
            TestCase {
                class_detail: String::from("3 sks / Sem 5 / BS"),
                lecturers: vec!["BS"],
            },
        ];

        test_cases.into_iter().for_each(|test| {
            assert_eq!(
                Excel::parse_lecturer(&test.class_detail),
                Some(test.lecturers)
            )
        });
    }

    #[test]
    fn test_parse_session() {
        struct TestCase {
            session_str: String,
            session_start: Option<String>,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                session_str: String::from("08.00 - 09.00"),
                session_start: Some(String::from("08.00")),
            },
            TestCase {
                session_str: String::from("17.00 - 18.00"),
                session_start: Some(String::from("17.00")),
            },
            TestCase {
                session_str: String::from("10.00 - 11.00"),
                session_start: Some(String::from("10.00")),
            },
        ];

        test_cases.into_iter().for_each(|test| {
            assert_eq!(Excel::parse_session(&test.session_str), test.session_start)
        })
    }
}
