pub mod db;
pub mod excel;

pub struct Class {
    matkul_id: String,
    lecture_id: String,
    day: String,
    code: String,
    session_id: u32,
}
struct Subject {
    id: String,
    name: String,
}
struct Lecturer {
    id: String,
    code: String,
}
struct Session {
    id: u32,
    session_time: String,
}
