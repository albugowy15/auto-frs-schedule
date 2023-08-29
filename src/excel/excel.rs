use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use calamine::{open_workbook, Reader, Xlsx};

use super::Excel;

impl Excel {
    pub fn new(
        file_path: &PathBuf,
        sheet_name: &str,
        list_subject: HashMap<String, String>,
        list_lecture: HashMap<String, String>,
        list_session: HashMap<String, i8>,
    ) -> Result<Self> {
        let mut excel: Xlsx<_> =
            open_workbook(file_path).with_context(|| "Cannot open excel file")?;
        let range = excel
            .worksheet_range(sheet_name)
            .context("Error opening sheet, make sure sheet name is exists")?
            .with_context(|| format!("Could not read excel range from sheet {}", sheet_name))?;
        Ok(Self {
            range,
            list_lecture,
            list_session,
            list_subject,
        })
    }
}
