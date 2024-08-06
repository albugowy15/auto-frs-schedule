pub mod find_class;
pub mod retrieve;

use std::path::PathBuf;

use anyhow::Context;
use calamine::{open_workbook, Data, Range, Reader, Xlsx};

use crate::db::repository::LecturerSubjectSessionMap;

pub struct Excel {
    pub range: Range<Data>,
    pub lecturer_subjects_session_map: LecturerSubjectSessionMap,
}

impl Excel {
    pub fn new(file_path: &PathBuf, sheet_name: &str) -> anyhow::Result<Self> {
        let mut excel: Xlsx<_> =
            open_workbook(file_path).with_context(|| "Cannot open excel file")?;
        let range = excel.worksheet_range(sheet_name)?;
        Ok(Self {
            range,
            lecturer_subjects_session_map: LecturerSubjectSessionMap::default(),
        })
    }

    pub fn with_repo_data(
        mut self,
        lecturer_subjects_session_map: LecturerSubjectSessionMap,
    ) -> Excel {
        self.lecturer_subjects_session_map = lecturer_subjects_session_map;
        self
    }
}
