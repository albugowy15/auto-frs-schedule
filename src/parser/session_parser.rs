use crate::excel::retrieve::Retrieve;
use crate::excel::Excel;
use crate::parser::Parser;

pub trait SessionParser<T> {
    fn get_session(&self, row_idx: u32) -> Option<T>;
}

impl SessionParser<i8> for Excel {
    fn get_session(&self, row_idx: u32) -> Option<i8> {
        let session_str = self.retrieve_session(row_idx)?;
        let session_name = Excel::parse_session(&session_str)?;
        self.lecturer_subjects_session_map
            .sessions
            .get(&session_name)
            .cloned()
    }
}

impl SessionParser<String> for Excel {
    fn get_session(&self, row_idx: u32) -> Option<String> {
        let session_str = self.retrieve_session(row_idx)?;
        let session_name = Excel::parse_session(&session_str)?;
        self.lecturer_subjects_session_map
            .sessions
            .contains_key(&session_name)
            .then_some(session_name)
    }
}
