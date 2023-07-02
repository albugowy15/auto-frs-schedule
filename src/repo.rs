use anyhow::{Context, Result};
use sqlx::{MySql, Pool, Row};
use std::collections::HashMap;

pub struct ClassRepository {}

#[derive(Debug)]
pub struct Class {
    pub matkul_id: String,
    pub lecture_id: String,
    pub day: String,
    pub code: String,
    pub session_id: i8,
}

impl ClassRepository {
    pub async fn get_all_subject(pool: &Pool<MySql>) -> Result<HashMap<String, String>> {
        let mut subjects = HashMap::new();
        let rows = sqlx::query("SELECT id, name FROM Matkul")
            .fetch_all(pool)
            .await
            .with_context(|| "Error executing get_all_subject sql")?;

        rows.into_iter().for_each(|subject| {
            subjects.insert(subject.get("name"), subject.get("id"));
        });
        Ok(subjects)
    }

    pub async fn get_all_lecture(pool: &Pool<MySql>) -> Result<HashMap<String, String>> {
        let mut lecturers = HashMap::new();
        let rows = sqlx::query("SELECT id, code FROM Lecturer")
            .fetch_all(pool)
            .await
            .with_context(|| "Error executing get_all_lecturer sql")?;

        rows.into_iter().for_each(|subject| {
            lecturers.insert(subject.get("code"), subject.get("id"));
        });
        Ok(lecturers)
    }

    pub async fn get_all_session(pool: &Pool<MySql>) -> Result<HashMap<String, i8>> {
        let mut sessions = HashMap::new();

        let rows = sqlx::query("SELECT id, session_time FROM Session")
            .fetch_all(pool)
            .await
            .with_context(|| "Error executing get_all_session sql")?;

        rows.into_iter().for_each(|session| {
            let session_start: String = session.get("session_time");
            sessions.insert(
                session_start.split("-").collect::<Vec<&str>>()[0].to_string(),
                session.get("id"),
            );
        });
        Ok(sessions)
    }

    #[allow(deprecated)]
    pub async fn insert_data(pool: &Pool<MySql>, data: &Vec<Class>) -> Result<()> {
        sqlx::query("DELETE FROM Plan")
            .execute(pool)
            .await
            .with_context(|| "Could not delete all Plan")?;
        sqlx::query("DELETE FROM _ClassToPlan")
            .execute(pool)
            .await
            .with_context(|| "Could not delete all _ClassToPlan")?;
        sqlx::query("DELETE FROM Class")
            .execute(pool)
            .await
            .with_context(|| "Could not delete all Class")?;

        let prep_sql = "INSERT INTO Class (id, matkulId, lecturerId, day, code, isAksel, taken, sessionId) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

        for item in data.iter() {
            let id_class = cuid::cuid().with_context(|| format!("Could not create cuid"))?;
            sqlx::query(prep_sql)
                .bind(id_class)
                .bind(&item.matkul_id)
                .bind(&item.lecture_id)
                .bind(&item.day)
                .bind(&item.code)
                .bind(false)
                .bind(0)
                .bind(&item.session_id)
                .execute(pool)
                .await
                .with_context(|| {
                    format!("Could not insert to Class table with statement {:?}", &item)
                })?;
        }
        Ok(())
    }
}
