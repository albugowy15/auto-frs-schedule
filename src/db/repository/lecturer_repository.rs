use std::collections::HashMap;

use anyhow::Result;
use sqlx::{MySql, Pool, Row};

use super::Repository;

pub struct LecturerRepository<'a> {
    db_pool: &'a Pool<MySql>,
}

impl<'a> Repository<'a> for LecturerRepository<'a> {
    fn new(db_pool: &'a Pool<MySql>) -> Self {
        LecturerRepository { db_pool }
    }
}

impl LecturerRepository<'_> {
    pub async fn get_all_lecture(&self) -> Result<HashMap<String, String>> {
        let rows = sqlx::query("SELECT id, code FROM Lecturer")
            .fetch_all(self.db_pool)
            .await?;
        let lecturers = rows
            .into_iter()
            .map(|lecturer| (lecturer.get("code"), lecturer.get("id")))
            .collect();
        Ok(lecturers)
    }
}
