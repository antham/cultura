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
        let query = "CREATE TABLE facts (fact TEXT UNIQUE, provider TEXT, was_displayed TINYINT(1), created_at TEXT);";
        match self.connection.execute(query, ()) {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    pub fn create(&self, provider: String, facts: Vec<String>) {
        facts.into_iter().for_each(|f| {
            match self.connection.execute(
                "INSERT INTO facts VALUES (?1, ?2, ?3, ?4);",
                [
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

    pub fn get_random_fact(&self) -> Option<String> {
        let mut stmt = self
            .connection
            .prepare(
                "SELECT fact FROM facts WHERE was_displayed = 0 ORDER BY created_at DESC LIMIT 1",
            )
            .unwrap();
        let result = stmt
            .query_map([(provider)], |row: &Row| {
                Ok(row.get::<usize, String>(0).unwrap())
            })
            .unwrap()
            .next()
            .unwrap();
        match result {
            Ok(s) => Some(s),
            _ => None,
        }
    }
}
