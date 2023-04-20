use chrono::Local;
use rusqlite::{Connection, Row};

pub struct Fact {
    connection: Connection,
}

impl Fact {
    pub fn new(path: &str) -> Self {
        let f = Fact {
            connection: Connection::open(path).unwrap(),
        };
        f.migrate();
        f
    }

    fn migrate(&self) {
        let query = "CREATE TABLE facts (id TEXT UNIQUE, fact TEXT UNIQUE, provider TEXT, was_displayed TINYINT(1), created_at TEXT);";
        match self.connection.execute(query, ()) {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    pub fn create(&self, provider: String, facts: Vec<String>) {
        facts.into_iter().for_each(|f| {
            match self.connection.execute(
                "INSERT INTO facts VALUES (?1, ?2, ?3, ?4, ?5);",
                [
                    (uuid::Uuid::new_v4().to_string()),
                    (f),
                    (provider.to_owned()),
                    (0.to_string()),
                    (Local::now().to_string()),
                ],
            ) {
                Ok(_) => (),
                Err(_) => (),
            }
        });
    }

    pub fn get_random_fact(&self) -> Option<(String, String)> {
        let mut stmt = self
            .connection
            .prepare(
                "SELECT id, fact FROM facts WHERE was_displayed = 0 ORDER BY created_at DESC LIMIT 1",
            )
            .unwrap();
        match stmt.query_row([], |row: &Row| {
            Ok((
                row.get::<usize, String>(0).unwrap(),
                row.get::<usize, String>(1).unwrap(),
            ))
        }) {
            Ok((id, fact)) => Some((id, fact)),
            Err(_) => None,
        }
    }

    pub fn mark_as_read(&self, id: String) -> Result<usize, rusqlite::Error> {
        self.connection
            .execute("UPDATE facts SET was_displayed = 1 WHERE id = ?1", [(id)])
    }
}
