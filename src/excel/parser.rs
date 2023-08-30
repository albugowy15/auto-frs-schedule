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
    fn test_parse_subject() {
        assert_eq!(
            Excel::parse_subject_with_code("Jaringan Komputer A"),
            Some(("Jaringan Komputer".to_string(), "A".to_string()))
        );

        assert_eq!(
            Excel::parse_subject_with_code("Realitas X"),
            Some(("Realitas X".to_string(), "-".to_string()))
        );
        assert_eq!(
            Excel::parse_subject_with_code("Interaksi Manusia Komputer D - EN"),
            Some((
                "Interaksi Manusia Komputer".to_string(),
                "D - EN".to_string()
            ))
        );
        assert_eq!(
            Excel::parse_subject_with_code("Interaksi Manusia Komputer - RKA"),
            Some(("Interaksi Manusia Komputer".to_string(), "RKA".to_string()))
        );

        assert_eq!(
            Excel::parse_subject_with_code("Jaringan Komputer - IUP"),
            Some(("Jaringan Komputer".to_string(), "IUP".to_string()))
        );
        assert_eq!(
            Excel::parse_subject_with_code("Dasar Pemrograman F"),
            Some(("Dasar Pemrograman".to_string(), "F".to_string()))
        );
    }
}
