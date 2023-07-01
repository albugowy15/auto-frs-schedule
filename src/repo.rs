use anyhow::{Context, Result};
use mysql_async::prelude::{Query, Queryable, WithParams};
use std::collections::HashMap;

pub struct ClassRepository {}

#[derive(Debug)]
pub struct Class {
    pub matkul_id: String,
    pub lecture_id: String,
    pub day: String,
    pub code: String,
    pub session_id: u32,
}

impl ClassRepository {
    pub async fn get_all_subject(conn: &mut mysql_async::Conn) -> Result<HashMap<String, String>> {
        struct Subject {
            id: String,
            name: String,
        }
        let loaded_subject = "SELECT id, name FROM Matkul"
            .with(())
            .map(conn, |(id, name)| Subject { id, name })
            .await
            .with_context(|| format!("Error executing get_al_subject sql"))?;
        let mut subjects = HashMap::new();

        loaded_subject.into_iter().for_each(|subject| {
            subjects.insert(subject.name, subject.id);
        });
        Ok(subjects)
    }

    pub async fn get_all_lecture(conn: &mut mysql_async::Conn) -> Result<HashMap<String, String>> {
        struct Lecturer {
            id: String,
            code: String,
        }
        let loaded_lecture = "SELECT id, code FROM Lecturer"
            .with(())
            .map(conn, |(id, code)| Lecturer { id, code })
            .await
            .with_context(|| format!("Error executing get_all_lecturer sql"))?;
        let mut lecturers = HashMap::new();

        loaded_lecture.into_iter().for_each(|lecture| {
            lecturers.insert(lecture.code, lecture.id);
        });
        Ok(lecturers)
    }

    pub async fn get_all_session(conn: &mut mysql_async::Conn) -> Result<HashMap<String, u32>> {
        struct Session {
            id: u32,
            session_time: String,
        }
        let loaded_session = "SELECT id, session_time FROM Session"
            .with(())
            .map(conn, |(id, session_time)| Session { id, session_time })
            .await
            .with_context(|| format!("Error executing get_all_session sql"))?;
        let mut sessions = HashMap::new();

        loaded_session.into_iter().for_each(|session| {
            let session_start = session.session_time.split("-").collect::<Vec<&str>>()[0];
            sessions.insert(session_start.to_string(), session.id);
        });
        Ok(sessions)
    }

    #[allow(deprecated)]
    pub async fn insert_data(conn: &mut mysql_async::Conn, data: &Vec<Class>) -> Result<()> {
        conn.query_drop("DELETE FROM Plan")
            .await
            .with_context(|| format!("Could not delete all Plan"))?;
        conn.query_drop("DELETE FROM _ClassToPlan")
            .await
            .with_context(|| format!("Could not delete all _ClassToPlan"))?;
        conn.query_drop("DELETE FROM Class")
            .await
            .with_context(|| format!("Could not delete all Class"))?;
        let prepared_stmt = conn.prep("INSERT INTO Class (id, matkulId, lecturerId, day, code, isAksel, taken, sessionId) VALUES (?, ?, ?, ?, ?, ?, ?, ?)").await.with_context(|| format!("Error preparing insert sql statement"))?;
        for item in data.iter() {
            let id_class = cuid::cuid().with_context(|| format!("Could not create cuid"))?;
            let values = (
                id_class.to_string(),
                &item.matkul_id,
                &item.lecture_id,
                &item.day,
                &item.code,
                false,
                0,
                &item.session_id,
            );
            conn.exec_drop(&prepared_stmt, &values)
                .await
                .with_context(|| {
                    format!(
                        "Could not insert to Class table with statement {:?}",
                        values
                    )
                })?;
        }
        Ok(())
    }
}
