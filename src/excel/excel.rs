use std::path::PathBuf;

use anyhow::{Context, Result};
use calamine::{open_workbook, Reader, Xlsx};

use super::Excel;

impl Excel {
    pub fn new(file_path: &PathBuf, sheet_name: &str) -> Result<Self> {
        let mut excel: Xlsx<_> =
            open_workbook(file_path).with_context(|| "Cannot open excel file")?;
        let range = excel
            .worksheet_range(sheet_name)
            .context("Error opening sheet, make sure sheet name is exists")?
            .with_context(|| format!("Could not read excel range from sheet {}", sheet_name))?;
        Ok(Self { range })
    }
}

#[cfg(test)]
mod test {
    use crate::excel::{Excel, IntoMap};
    use std::collections::HashMap;

    #[test]
    fn test_parse_subject() {
        let mut subject_map: HashMap<String, String> = HashMap::new();
        subject_map.insert(String::from("jaringan komputer"), String::from("c636gggdd"));
        assert_eq!(
            Excel::subject_class_to_map("Jaringan Komputer A", &subject_map),
            Some(("c636gggdd".to_string(), "A".to_string()))
        );

        subject_map.insert(String::from("realitas x"), String::from("377hh7cch"));
        assert_eq!(
            Excel::subject_class_to_map("Realitas X", &subject_map),
            Some(("377hh7cch".to_string(), "-".to_string()))
        );
        subject_map.insert(
            String::from("interaksi manusia komputer"),
            String::from("wjjfhhfw888"),
        );
        assert_eq!(
            Excel::subject_class_to_map("Interaksi Manusia Komputer D - EN", &subject_map),
            Some(("wjjfhhfw888".to_string(), "D - EN".to_string()))
        );
        assert_eq!(
            Excel::subject_class_to_map("Interaksi Manusia Komputer - RKA", &subject_map),
            Some(("wjjfhhfw888".to_string(), "RKA".to_string()))
        );

        assert_eq!(
            Excel::subject_class_to_map("Jaringan Komputer - IUP", &subject_map),
            Some(("c636gggdd".to_string(), "IUP".to_string()))
        );

        subject_map.insert(String::from("dasar pemrograman"), String::from("cc773hhe"));
        assert_eq!(
            Excel::subject_class_to_map("Dasar Pemrograman F", &subject_map),
            Some(("cc773hhe".to_string(), "F".to_string()))
        );
    }
}
