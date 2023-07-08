use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use sqlx::{MySql, Pool, Row};
use std::collections::HashMap;

pub struct ClassRepository;

#[derive(Debug)]
pub struct Class {
    pub matkul_id: String,
    pub lecturers_id: Vec<String>,
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

        rows.into_iter().for_each(|lecturer| {
            lecturers.insert(lecturer.get("code"), lecturer.get("id"));
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
            sessions.insert(
                session
                    .get::<String, &str>("session_time")
                    .split("-")
                    .collect::<Vec<&str>>()[0]
                    .to_string(),
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
        sqlx::query("DELETE FROM _ClassToLecturer")
            .execute(pool)
            .await
            .with_context(|| "Could not delete all _ClassToLecturer")?;

        let prep_class_sql = "INSERT INTO Class (id, matkulId, day, code, isAksel, taken, sessionId) VALUES (?, ?, ?, ?, ?, ?, ?)";
        let prep_class_lecturers_sql = "INSERT INTO _ClassToLecturer (A, B) VALUES (?, ?)";

        let bar = ProgressBar::new(data.len() as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
            )
            .unwrap()
            .progress_chars("#>-"),
        );

        for item in data.iter() {
            let id_class = cuid::cuid().with_context(|| "Could not create cuid")?;

            for lec in item.lecturers_id.iter() {
                sqlx::query(prep_class_lecturers_sql)
                    .bind(&id_class)
                    .bind(&lec)
                    .execute(pool)
                    .await
                    .with_context(|| {
                        format!(
                            "Could not insert to _ClassToLecturer table with statement {:?}",
                            &lec
                        )
                    })?;
            }

            sqlx::query(prep_class_sql)
                .bind(&id_class)
                .bind(&item.matkul_id)
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
            bar.inc(1);
        }
        bar.finish_with_message(format!(
            "Done inserting {} classes to Class table",
            data.len()
        ));
        Ok(())
    }
}
