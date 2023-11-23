use std::collections::HashMap;

use anyhow::Result;
use sqlx::{MySql, Pool, Row};

use super::Repository;

pub struct SubjectRepository<'a> {
    db_pool: &'a Pool<MySql>,
}

impl<'a> Repository<'a> for SubjectRepository<'a> {
    fn new(db_pool: &'a Pool<MySql>) -> Self {
        SubjectRepository { db_pool }
    }
}

impl SubjectRepository<'_> {
    pub fn new(db_pool: &Pool<MySql>) -> SubjectRepository {
        SubjectRepository { db_pool }
    }
    pub async fn get_all_subject(&self) -> Result<HashMap<String, String>> {
        let rows = sqlx::query("SELECT id, name FROM Matkul")
            .fetch_all(self.db_pool)
            .await?;

        let subjects = rows
            .into_iter()
            .map(|subject| {
                (
                    subject.get::<String, _>("name").to_lowercase(),
                    subject.get("id"),
                )
            })
            .collect();

        Ok(subjects)
    }
}
