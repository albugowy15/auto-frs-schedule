use std::collections::HashMap;

use anyhow::Result;
use sqlx::{MySql, Pool, Row};

pub struct SessionRepository<'a> {
    db_pool: &'a Pool<MySql>,
}

impl SessionRepository<'_> {
    pub fn new(db_pool: &Pool<MySql>) -> SessionRepository {
        SessionRepository { db_pool }
    }

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
