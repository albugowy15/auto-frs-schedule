use crate::excel::Excel;
use calamine::DataType;

pub trait Retrieve {
    fn retrieve_class_detail(&self, row_idx: u32, col_idx: u32) -> Option<String>;
    fn retrieve_session(&self, row_idx: u32) -> Option<String>;
}

impl Retrieve for Excel {
    fn retrieve_class_detail(&self, row_idx: u32, col_idx: u32) -> Option<String> {
        self.range
            .get_value((row_idx + 1, col_idx))
            .and_then(|cell| cell.get_string())
            .map(|lecturer| lecturer.to_string())
    }

    fn retrieve_session(&self, row_idx: u32) -> Option<String> {
        self.range
            .get_value((row_idx, 1))
            .and_then(|cell| cell.get_string())
            .map(|s| s.to_owned())
    }
}
