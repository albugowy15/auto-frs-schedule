use anyhow::Result;
use sqlx::{MySql, Pool};
use std::collections::HashMap;

use crate::db::repository::{
    lecturer::LecturerRepository, session::SessionRepository, subject::SubjectRepository,
};

pub mod class;
pub mod lecturer;
pub mod many_to_many;
pub mod plan;
pub mod session;
pub mod subject;

pub trait Repository<'a> {
    fn new(db_pool: &'a Pool<MySql>) -> Self;
}

#[derive(Default)]
pub struct LecturerSubjectSessionMap {
    pub subjects: HashMap<String, String>,
    pub lecturers: HashMap<String, String>,
    pub sessions: HashMap<String, i8>,
}

impl LecturerSubjectSessionMap {
    pub fn new() -> Self {
        Self {
            subjects: HashMap::new(),
            lecturers: HashMap::new(),
            sessions: HashMap::new(),
        }
    }
}

pub async fn prepare_data(pool: &Pool<MySql>) -> Result<LecturerSubjectSessionMap> {
    println!("Get all subjects from DB");
    let lecturer_repo = LecturerRepository::new(pool);
    let subject_repo = SubjectRepository::new(pool);
    let session_repo = SessionRepository::new(pool);
    let (subjects, lecturers, sessions) = tokio::try_join!(
        subject_repo.get_all_subjects(),
        lecturer_repo.get_all_lecturers(),
        session_repo.get_all_sessions()
    )?;
    Ok(LecturerSubjectSessionMap {
        subjects,
        lecturers,
        sessions,
    })
}
