use std::collections::HashMap;

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use sqlx::{MySql, Pool, Row};

use super::Repository;

pub struct ClassRepository<'a> {
    db_pool: &'a Pool<MySql>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub matkul_id: String,
    pub lecturers_id: Vec<String>,
    pub day: String,
    pub code: String,
    pub session_id: i8,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ClassFromSchedule {
    pub subject_name: String,
    pub class_code: String,
    pub lecturer_code: Vec<String>,
    pub day: String,
    pub session_start: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ClassFindSchedule {
    pub class: String,
    pub lecturers_code: Vec<String>,
    pub day: String,
    pub session_start: String,
}

impl<'a> Repository<'a> for ClassRepository<'a> {
    fn new(db_pool: &'a Pool<MySql>) -> Self {
        ClassRepository { db_pool }
    }
}

impl ClassRepository<'_> {
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
        let non_classes = [
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
            let matkul_id: &str = res
                .try_get("id")
                .with_context(|| "Error getting matkul_id")?;
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
    pub async fn insert_classes(&self, data: &[Class]) -> Result<()> {
        let mut tx = self.db_pool.begin().await?;
        Self::drop_old_classes(&mut tx)
            .await
            .with_context(|| "Error dropping old classes")?;

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
                    .bind(lec)
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
                .bind(item.session_id)
                .execute(&mut *tx)
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
        log::info!("Inserting non-class subject");
        Self::insert_non_classes(&mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_schedule(&self) -> Result<HashMap<(String, String), ClassFromSchedule>> {
        let rows = sqlx::query(
            "SELECT c.id, m.name as subject_name, c.code as class_code, c.day, l.code as lecture_code, cls.total_lecturer, s.session_time FROM Class c INNER JOIN (SELECT c.id, COUNT(c.id) as total_lecturer  FROM Class c INNER JOIN `_ClassToLecturer` ctl ON c.id = ctl.A INNER JOIN Lecturer l ON ctl.B = l.id GROUP BY (c.id)) cls ON cls.id = c.id INNER JOIN Matkul m ON c.matkulId = m.id INNER JOIN Session s on s.id = c.sessionId INNER JOIN `_ClassToLecturer` ctl ON c.id = ctl.A INNER JOIN Lecturer l ON ctl.B = l.id",
        )
        .fetch_all(self.db_pool)
        .await?;

        let mut class_map = HashMap::new();
        for row in rows.into_iter() {
            let total_lecturer = row.get::<i32, _>("total_lecturer");
            let lecturer_code: Vec<String> = if total_lecturer > 1 {
                let class_id: String = row.get("id");
                let lec_rows = sqlx::query("SELECT l.code FROM Lecturer l INNER JOIN `_ClassToLecturer` ctl ON l.id = ctl.B INNER JOIN Class c ON ctl.A = c.id WHERE c.id = ?")
                    .bind(class_id)
                    .fetch_all(self.db_pool).await?;
                lec_rows.into_iter().map(|row| row.get("code")).collect()
            } else {
                vec![row.get("lecture_code")]
            };
            let key: (String, String) = (row.get("subject_name"), row.get("class_code"));
            let session_start: String = row.get("session_time");
            let value = ClassFromSchedule {
                class_code: row.get("class_code"),
                day: row.get("day"),
                lecturer_code,
                session_start: session_start
                    .split('-')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string(),
                subject_name: row.get("subject_name"),
            };
            class_map.insert(key, value);
        }
        Ok(class_map)
    }

    pub async fn sync_taken(&self) -> Result<()> {
        let mut tx = self.db_pool.begin().await?;
        let rows = sqlx::query(
            "select count(p.id) as actual_taken, c.taken, c.id from Class c inner join _ClassToPlan cp on cp.A = c.id inner join Plan p on cp.B = p.id group by c.id having count(p.id) != c.taken",
        )
        .fetch_all(&mut *tx)
        .await?;
        log::info!("Sync taken {} classes", rows.len());

        for row in rows.into_iter() {
            let actual_class_taken: i8 = row.get("actual_taken");
            let class_id: String = row.get("id");
            sqlx::query("update Class set taken = ? where id = ?")
                .bind(actual_class_taken)
                .bind(class_id)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
