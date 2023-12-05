use super::{Excel, Parser};

impl Parser for Excel {
    fn parse_lecturer(&self, row: u32, col: u32) -> Option<Vec<&str>> {
        self.range
            .get_value((row + 1, col))
            .and_then(|cell| cell.get_string())
            .map(|lecturer| {
                lecturer
                    .split('/')
                    .nth(2)
                    .unwrap_or_default()
                    .split('-')
                    .collect()
            })
    }

    fn parse_session(&self, row_idx: u32) -> Option<String> {
        self.range
            .get_value((row_idx, 1))
            .and_then(|cell| cell.get_string())
            .map(|session_str| {
                session_str
                    .split(" - ")
                    .next()
                    .unwrap_or_default()
                    .to_string()
            })
    }

    fn parse_subject_with_code_2(val: &str) -> Option<(String, String)> {
        // Different parse method for each kind of class code
        // CASE 1: (EN) + IUP
        if val.contains("(EN) + IUP") {
            return Some((
                val.split("(EN) + IUP").next()?.trim().to_string(),
                "(EN) + IUP".to_string(),
            ));
        }

        // CASE 2: IUP
        if val.contains("IUP") {
            let subject = val.split("IUP").next()?.trim().split('-').next()?.trim();
            // let subject = splitted[0..splitted.len() - 1].join(" ");
            return Some((subject.to_string(), "IUP".to_string()));
        }

        // CASE 3: EN
        if val.contains("EN") {
            let splitted: Vec<&str> = val
                .split("EN")
                .next()?
                .trim()
                .split('-')
                .next()?
                .split_ascii_whitespace()
                .collect();
            let code = splitted.last()?.trim();
            let subject = splitted[0..splitted.len() - 1].join(" ");
            return Some((subject, format!("{} - EN", code)));
        }

        // CASE 4: - RKA, - RPL
        if val.contains('-') {
            let mut splitted = val.split('-').map(|x| x.trim());
            return Some((splitted.next()?.to_string(), splitted.next()?.to_string()));
        }

        // CASE 5: Unique Class, examples : Realitas X
        if val.contains("Realitas X") {
            return Some((val.trim().to_string(), "-".to_string()));
        }

        // CASE 6: Basic Class, examples: Jaringan Komputer A
        let splitted: Vec<&str> = val.split_ascii_whitespace().collect();
        if let Some(last_elm) = splitted.last() {
            if last_elm.len() == 1 {
                let code = last_elm;
                let subject = splitted[0..(splitted.len() - 1)].join(" ");
                return Some((subject, code.to_string()));
            } else {
                return Some((splitted.join(" "), "-".to_string()));
            }
        }
        None
    }

    fn parse_subject_with_code(val: &str) -> Option<(String, String)> {
        let splitted = val.split('-').collect::<Vec<&str>>();
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

        Some((subject_name, code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_subject_with_code_2() {
        struct TestCase {
            class: String,
            subject_name: String,
            subject_code: String,
        }
        let test_cases: Vec<TestCase> = vec![
            TestCase {
                class: "Interaksi Manusia Komputer (EN) + IUP".to_string(),
                subject_code: "(EN) + IUP".to_string(),
                subject_name: "Interaksi Manusia Komputer".to_string(),
            },
            TestCase {
                class: "Interaksi Manusia Komputer D - EN".to_string(),
                subject_code: "D - EN".to_string(),
                subject_name: "Interaksi Manusia Komputer".to_string(),
            },
            TestCase {
                class: "Interaksi Manusia Komputer D-EN".to_string(),
                subject_code: "D - EN".to_string(),
                subject_name: "Interaksi Manusia Komputer".to_string(),
            },
            TestCase {
                class: "Jaringan Komputer - IUP".to_string(),
                subject_code: "IUP".to_string(),
                subject_name: "Jaringan Komputer".to_string(),
            },
            TestCase {
                class: "Jaringan Komputer-IUP".to_string(),
                subject_code: "IUP".to_string(),
                subject_name: "Jaringan Komputer".to_string(),
            },
            TestCase {
                class: "Interaksi Manusia Komputer - RKA".to_string(),
                subject_code: "RKA".to_string(),
                subject_name: "Interaksi Manusia Komputer".to_string(),
            },
            TestCase {
                class: "Interaksi Manusia Komputer-RPL".to_string(),
                subject_code: "RPL".to_string(),
                subject_name: "Interaksi Manusia Komputer".to_string(),
            },
            TestCase {
                class: "Game Cerdas".to_string(),
                subject_code: "-".to_string(),
                subject_name: "Game Cerdas".to_string(),
            },
            TestCase {
                class: "Realitas X".to_string(),
                subject_code: "-".to_string(),
                subject_name: "Realitas X".to_string(),
            },
            TestCase {
                class: "Jaringan Komputer A".to_string(),
                subject_code: "A".to_string(),
                subject_name: "Jaringan Komputer".to_string(),
            },
        ];
        test_cases.into_iter().for_each(|case| {
            assert_eq!(
                Excel::parse_subject_with_code_2(&case.class),
                Some((case.subject_name, case.subject_code))
            );
        });
    }
}
