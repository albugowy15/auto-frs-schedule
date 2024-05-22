pub mod commands;
pub mod db;
pub mod excel;
pub mod file_writer;
pub mod parser;

pub const DAYS: [&str; 5] = ["Senin", "Selasa", "Rabu", "Kamis", "Jum'at"];
pub const DAY_OFFSET: usize = 14;
