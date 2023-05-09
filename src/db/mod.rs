use chrono::Local;
use rusqlite::Connection;

pub struct Fact {
    connection: Connection,
}

impl Fact {
    pub fn new(path: &str) -> Result<Self, String> {
        match Connection::open(path) {
            Ok(connection) => {
                let f = Fact { connection };
                match f.migrate() {
                    Ok(_) => Ok(f),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(format!("cannot open {} : {}", path, e)),
        }
    }

    fn migrate(&self) -> Result<(), String> {
        let query = "CREATE TABLE IF NOT EXISTS facts (id TEXT UNIQUE, fact TEXT UNIQUE, provider TEXT, was_displayed TINYINT(1), created_at TEXT);";
        match self.connection.execute(query, ()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn create(&self, provider: String, facts: Vec<String>) -> Vec<Result<(), String>> {
        facts
            .into_iter()
            .map(|f| {
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
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            })
            .collect::<Vec<Result<(), String>>>()
    }

    pub fn get_random_fact(&self) -> Result<Option<(String, String)>, String> {
        let mut stmt = self
            .connection
            .prepare(
                "SELECT id, fact FROM facts WHERE was_displayed = 0 ORDER BY created_at DESC LIMIT 1",
            )
            .unwrap();

        let mut rows = stmt.query([]).unwrap();

        let mut results = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            results.push(Some((
                row.get::<usize, String>(0).unwrap(),
                row.get::<usize, String>(1).unwrap(),
            )))
        }
        Ok(results.pop().unwrap_or(None))
    }

    pub fn mark_as_read(&self, id: String) -> Result<(), String> {
        match self
            .connection
            .execute("UPDATE facts SET was_displayed = 1 WHERE id = ?1", [(id)])
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
