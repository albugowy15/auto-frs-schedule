use std::collections::HashMap;

use anyhow::Result;
use sqlx::{MySql, Pool, Row};

use super::Repository;

pub struct SessionRepository<'a> {
    db_pool: &'a Pool<MySql>,
}

impl<'a> Repository<'a> for SessionRepository<'a> {
    fn new(db_pool: &'a Pool<MySql>) -> Self {
        SessionRepository { db_pool }
    }
}

impl SessionRepository<'_> {
    pub async fn get_all_session(&self) -> Result<HashMap<String, i8>> {
        let rows = sqlx::query("SELECT id, session_time FROM Session")
            .fetch_all(self.db_pool)
            .await?;
        let sessions = rows
            .into_iter()
            .map(|session| {
                (
                    session
                        .get::<String, &str>("session_time")
                        .split('-')
                        .collect::<Vec<_>>()[0]
                        .to_string(),
                    session.get("id"),
                )
            })
            .collect();
        Ok(sessions)
    }
}
