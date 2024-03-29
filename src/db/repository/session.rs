use std::collections::HashMap;

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
    pub async fn get_all_sessions(&self) -> Result<HashMap<String, i8>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, session_time FROM Session")
            .fetch_all(self.db_pool)
            .await?;
        let mut sessions = HashMap::with_capacity(rows.len());
        for row in rows.into_iter() {
            let session_time = row.try_get::<String, &str>("session_time")?;
            let first_part = session_time.split('-').next().unwrap_or("").to_string();
            sessions.insert(first_part, row.try_get("id")?);
        }
        // let sessions = rows
        //     .into_iter()
        //     .map(|session| {
        //         let session_time = session.get::<String, &str>("session_time");
        //         let first_part = session_time.split('-').next().unwrap_or("").to_string();
        //         (first_part, session.get("id"))
        //     })
        //     .collect();
        Ok(sessions)
    }
}
