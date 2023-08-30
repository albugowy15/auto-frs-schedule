use super::{Excel, Parser};

impl Parser for Excel {
    fn parse_lecturer(&self, row: u32, col: u32) -> Option<Vec<&str>> {
        let lecturer = self
            .range
            .get_value((row + 1, col))?
            .get_string()?
            .split("/")
            .collect::<Vec<_>>()[2]
            .split("-")
            .collect();
        Some(lecturer)
    }

    fn parse_session(&self, row_idx: u32) -> Option<String> {
        let session_name = self
            .range
            .get_value((row_idx, 1))?
            .get_string()?
            .split(" - ")
            .collect::<Vec<&str>>()[0]
            .to_string();
        Some(session_name)
    }

    fn parse_subject_with_code_2(val: &str) -> Option<(String, String)> {
        // Different parse method for each kind of class code
        // CASE 1: (EN) + IUP
        if val.contains("(EN) + IUP") {
            return Some((
                val.split("(EN) + IUP").nth(0)?.trim().to_string(),
                "(EN) + IUP".to_string(),
            ));
        }

        // CASE 2: IUP
        if val.contains("IUP") {
            let subject = val.split("IUP").nth(0)?.trim().split("-").nth(0)?.trim();
            // let subject = splitted[0..splitted.len() - 1].join(" ");
            return Some((subject.to_string(), "IUP".to_string()));
        }

        // CASE 3: EN
        if val.contains("EN") {
            let splitted: Vec<&str> = val
                .split("EN")
                .nth(0)?
                .trim()
                .split("-")
                .nth(0)?
                .split_ascii_whitespace()
                .collect();
            let code = splitted.last()?.trim();
            let subject = splitted[0..splitted.len() - 1].join(" ");
            return Some((subject, format!("{} - EN", code)));
        }

        // CASE 4: - RKA, - RPL
        if val.contains("-") {
            let splitted: Vec<&str> = val.split("-").map(|x| x.trim()).collect();
            return Some((splitted[0].to_string(), splitted[1].to_string()));
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

        Some((subject_name, code))
    }
}

#[cfg(test)]
mod test {
    use crate::excel::{Excel, Parser};

    #[test]
    fn test_parse_en_iup_class() {
        assert_eq!(
            Excel::parse_subject_with_code_2("Interaksi Manusia Komputer (EN) + IUP"),
            Some((
                "Interaksi Manusia Komputer".to_string(),
                "(EN) + IUP".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_en_class() {
        assert_eq!(
            Excel::parse_subject_with_code_2("Interaksi Manusia Komputer D - EN"),
            Some((
                "Interaksi Manusia Komputer".to_string(),
                "D - EN".to_string()
            ))
        );
        assert_eq!(
            Excel::parse_subject_with_code_2("Interaksi Manusia Komputer D -EN"),
            Some((
                "Interaksi Manusia Komputer".to_string(),
                "D - EN".to_string()
            ))
        );
        assert_eq!(
            Excel::parse_subject_with_code_2("Interaksi Manusia Komputer D-EN"),
            Some((
                "Interaksi Manusia Komputer".to_string(),
                "D - EN".to_string()
            ))
        );
    }

    #[test]
    fn test_parse_iup_class() {
        assert_eq!(
            Excel::parse_subject_with_code_2("Jaringan Komputer -IUP"),
            Some(("Jaringan Komputer".to_string(), "IUP".to_string()))
        );
        assert_eq!(
            Excel::parse_subject_with_code_2("Jaringan Komputer-IUP"),
            Some(("Jaringan Komputer".to_string(), "IUP".to_string()))
        );
        assert_eq!(
            Excel::parse_subject_with_code_2("Jaringan Komputer - IUP"),
            Some(("Jaringan Komputer".to_string(), "IUP".to_string()))
        );
    }

    #[test]
    fn test_parse_strip_class() {
        assert_eq!(
            Excel::parse_subject_with_code_2("Interaksi Manusia Komputer - RKA"),
            Some(("Interaksi Manusia Komputer".to_string(), "RKA".to_string()))
        );
        assert_eq!(
            Excel::parse_subject_with_code_2("Interaksi Manusia Komputer - RPL"),
            Some(("Interaksi Manusia Komputer".to_string(), "RPL".to_string()))
        );
        assert_eq!(
            Excel::parse_subject_with_code_2("Dasar Pemrograman -RPL"),
            Some(("Dasar Pemrograman".to_string(), "RPL".to_string()))
        );
        assert_eq!(
            Excel::parse_subject_with_code_2("Dasar Pemrograman-RPL"),
            Some(("Dasar Pemrograman".to_string(), "RPL".to_string()))
        );
    }

    #[test]
    fn test_parse_single_class() {
        assert_eq!(
            Excel::parse_subject_with_code_2("Game Cerdas"),
            Some(("Game Cerdas".to_string(), "-".to_string()))
        );
        assert_eq!(
            Excel::parse_subject_with_code_2("Realitas X"),
            Some(("Realitas X".to_string(), "-".to_string()))
        );
    }

    #[test]
    fn test_parse_basic_class() {
        assert_eq!(
            Excel::parse_subject_with_code_2("Jaringan Komputer A"),
            Some(("Jaringan Komputer".to_string(), "A".to_string()))
        );
    }
}
