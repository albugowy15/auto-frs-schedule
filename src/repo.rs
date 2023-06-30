use std::collections::HashMap;

use mysql_async::prelude::{Query, Queryable, WithParams};

pub struct ClassRepository {
    pub subjects: HashMap<String, String>,
    pub lecturers: HashMap<String, String>,
    pub sessions: HashMap<String, u32>,
}

#[derive(Debug)]
pub struct Class {
    pub matkul_id: String,
    pub lecture_id: String,
    pub day: String,
    pub code: String,
    pub session_id: u32,
}

impl ClassRepository {
    pub fn new() -> Self {
        Self {
            subjects: HashMap::new(),
            lecturers: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    pub async fn get_all_subject(
        &mut self,
        conn: &mut mysql_async::Conn,
    ) -> Result<(), mysql_async::Error> {
        struct Subject {
            id: String,
            name: String,
        }
        let loaded_subject = "SELECT id, name FROM Matkul"
            .with(())
            .map(conn, |(id, name)| Subject { id, name })
            .await?;

        loaded_subject.into_iter().for_each(|subject| {
            self.subjects.insert(subject.name, subject.id);
        });
        Ok(())
    }

    pub async fn get_all_lecture(
        &mut self,
        conn: &mut mysql_async::Conn,
    ) -> Result<(), mysql_async::Error> {
        struct Lecturer {
            id: String,
            code: String,
        }
        let loaded_lecture = "SELECT id, code FROM Lecturer"
            .with(())
            .map(conn, |(id, code)| Lecturer { id, code })
            .await?;

        loaded_lecture.into_iter().for_each(|lecture| {
            self.lecturers.insert(lecture.code, lecture.id);
        });
        Ok(())
    }

    pub async fn get_all_session(
        &mut self,
        conn: &mut mysql_async::Conn,
    ) -> Result<(), mysql_async::Error> {
        struct Session {
            id: u32,
            session_time: String,
        }
        let loaded_session = "SELECT id, session_time FROM Session"
            .with(())
            .map(conn, |(id, session_time)| Session { id, session_time })
            .await?;

        loaded_session.into_iter().for_each(|session| {
            let session_start = session.session_time.split("-").collect::<Vec<&str>>()[0];
            self.sessions.insert(session_start.to_string(), session.id);
        });
        Ok(())
    }

    #[allow(deprecated)]
    pub async fn insert_data(
        &self,
        conn: &mut mysql_async::Conn,
        data: Vec<Class>,
    ) -> Result<(), mysql_async::Error> {
        conn.query_drop("DELETE FROM Plan").await?;
        conn.query_drop("DELETE FROM _ClassToPlan").await?;
        conn.query_drop("DELETE FROM Class").await?;
        let prepared_stmt = conn.prep("INSERT INTO Class (id, matkulId, lecturerId, day, code, isAksel, taken, sessionId) VALUES (?, ?, ?, ?, ?, ?, ?, ?)").await?;
        for item in data.iter() {
            let id_class = cuid::cuid().unwrap();
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
            conn.exec_drop(&prepared_stmt, values).await?;
        }
        Ok(())
    }
}
