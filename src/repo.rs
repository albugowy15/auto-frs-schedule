use std::collections::HashMap;

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use sqlx::{MySql, MySqlPool, Pool, Row};

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
        let rows = sqlx::query("SELECT id, name FROM Matkul")
            .fetch_all(pool)
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

    pub async fn get_all_lecture(pool: &Pool<MySql>) -> Result<HashMap<String, String>> {
        let rows = sqlx::query("SELECT id, code FROM Lecturer")
            .fetch_all(pool)
            .await?;

        let lecturers = rows
            .into_iter()
            .map(|lecturer| (lecturer.get("code"), lecturer.get("id")))
            .collect();
        Ok(lecturers)
    }

    pub async fn get_all_session(pool: &Pool<MySql>) -> Result<HashMap<String, i8>> {
        let rows = sqlx::query("SELECT id, session_time FROM Session")
            .fetch_all(pool)
            .await?;
        let sessions = rows
            .into_iter()
            .map(|session| {
                (
                    session
                        .get::<String, &str>("session_time")
                        .split("-")
                        .collect::<Vec<_>>()[0]
                        .to_string(),
                    session.get("id"),
                )
            })
            .collect();
        Ok(sessions)
    }

    async fn drop_old_classes(transaction: &mut sqlx::Transaction<'_, sqlx::MySql>) -> Result<()> {
        sqlx::query("DELETE FROM Plan")
            .execute(&mut **transaction)
            .await?;
        sqlx::query("DELETE FROM _ClassToPlan")
            .execute(&mut **transaction)
            .await?;
        sqlx::query("DELETE FROM Class")
            .execute(&mut **transaction)
            .await?;
        sqlx::query("DELETE FROM _ClassToLecturer")
            .execute(&mut **transaction)
            .await?;
        Ok(())
    }

    #[allow(deprecated)]
    async fn insert_non_classes(
        transaction: &mut sqlx::Transaction<'_, sqlx::MySql>,
    ) -> Result<()> {
        let non_classes = vec![
            "Tugas Akhir",
            "Proposal Tugas Akhir",
            "Kerja Praktik",
            "Magang",
            "Etika Profesi",
        ];
        let class_stmt = "INSERT INTO Class (id, matkulId, code, taken) VALUES (?, ?, ?, ?)";
        for item in non_classes.iter() {
            let res = sqlx::query("SELECT id from Matkul WHERE name = ?")
                .bind(item)
                .fetch_one(&mut **transaction)
                .await
                .with_context(|| format!("Error find matkul_id for {}", item))?;
            let matkul_id: &str = res.get("id");
            let id_class = cuid::cuid()?;
            sqlx::query(class_stmt)
                .bind(&id_class)
                .bind(matkul_id)
                .bind("-")
                .bind(0)
                .execute(&mut **transaction)
                .await
                .with_context(|| format!("Error inserting {} {}", id_class, matkul_id))?;
        }
        Ok(())
    }
    #[allow(deprecated)]
    pub async fn insert_classes(pool: &MySqlPool, data: &Vec<Class>) -> Result<()> {
        let mut tx = pool.begin().await?;
        Self::drop_old_classes(&mut tx)
            .await
            .with_context(|| "Error drop old classes")?;

        let class_stmt = "INSERT INTO Class (id, matkulId, day, code, taken, sessionId) VALUES (?, ?, ?, ?, ?, ?)";
        let class_lecturers_stmt = "INSERT INTO _ClassToLecturer (A, B) VALUES (?, ?)";

        let bar = ProgressBar::new(data.len() as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
            )?
            .progress_chars("#>-"),
        );

        for item in data.iter() {
            let id_class = cuid::cuid()?;

            for lec in item.lecturers_id.iter() {
                sqlx::query(class_lecturers_stmt)
                    .bind(&id_class)
                    .bind(&lec)
                    .execute(&mut *tx)
                    .await
                    .with_context(|| {
                        format!(
                            "Could not insert to _ClassToLecturer table with statement {:?}",
                            &lec
                        )
                    })?;
            }

            sqlx::query(class_stmt)
                .bind(&id_class)
                .bind(&item.matkul_id)
                .bind(&item.day)
                .bind(&item.code)
                .bind(0)
                .bind(&item.session_id)
                .execute(&mut *tx)
                .await
                .with_context(|| {
                    format!("Could not insert to Class table with statement {:?}", &item)
                })?;
            bar.inc(1);
        }
        println!("Inserting non-class subject");
        Self::insert_non_classes(&mut tx).await?;
        tx.commit().await?;
        bar.finish_with_message(format!(
            "Done inserting {} classes to Class table",
            data.len()
        ));
        Ok(())
    }
}
