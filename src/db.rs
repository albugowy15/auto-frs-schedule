use std::collections::HashMap;

use mysql_async::{
    prelude::{Query, WithParams},
    Conn, Error, Pool,
};

struct Subject {
    id: String,
    name: String,
}

struct Lecturer {
    id: String,
    code: String,
}

struct Session {
    id: u32,
    session_time: String,
}

pub struct SQLData {
    pub subject: HashMap<String, String>,
    pub lecturer: HashMap<String, String>,
    pub session: HashMap<String, u32>,
}

impl SQLData {
    pub fn new() -> Self {
        Self {
            subject: HashMap::new(),
            lecturer: HashMap::new(),
            session: HashMap::new(),
        }
    }
    pub async fn get_all_subject(&mut self, conn: &mut Conn) -> Result<(), Error> {
        let loaded_subject = "SELECT id, name FROM Matkul"
            .with(())
            .map(conn, |(id, name)| Subject { id, name })
            .await?;

        loaded_subject.into_iter().for_each(|subject| {
            self.subject.insert(subject.name, subject.id);
        });

        Ok(())
    }

    pub async fn get_all_lecture(&mut self, conn: &mut Conn) -> Result<(), Error> {
        let loaded_lecture = "SELECT id, code FROM Lecturer"
            .with(())
            .map(conn, |(id, code)| Lecturer { id, code })
            .await?;

        loaded_lecture.into_iter().for_each(|lecture| {
            self.lecturer.insert(lecture.code, lecture.id);
        });

        Ok(())
    }

    pub async fn get_all_session(&mut self, conn: &mut Conn) -> Result<(), Error> {
        let loaded_session = "SELECT id, session_time FROM Session"
            .with(())
            .map(conn, |(id, session_time)| Session { id, session_time })
            .await?;

        loaded_session.into_iter().for_each(|session| {
            let session_start = session.session_time.split("-").collect::<Vec<&str>>()[0];
            self.session.insert(session_start.to_string(), session.id);
        });

        Ok(())
    }
}

pub async fn start_connection(database_url: String) -> Result<Pool, Error> {
    let builder =
        mysql_async::OptsBuilder::from_opts(mysql_async::Opts::from_url(&database_url).unwrap());

    return Ok(mysql_async::Pool::new(
        builder.ssl_opts(mysql_async::SslOpts::default()),
    ));
}
