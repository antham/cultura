use std::error::Error;

use chrono::Local;
use rusqlite::Connection;

pub struct Fact {
    connection: Connection,
}

impl Fact {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        Connection::open(path)
            .map(|connection| Fact { connection })
            .map(|f| -> Result<Self, Box<dyn Error>> {
                f.migrate()?;
                Ok(f)
            })?
    }

    fn migrate(&self) -> Result<(), Box<dyn Error>> {
        let query = "CREATE TABLE IF NOT EXISTS facts (id TEXT UNIQUE, fact TEXT UNIQUE, provider TEXT, was_displayed TINYINT(1), created_at TEXT);";
        self.connection.execute(query, ())?;
        Ok(())
    }

    pub fn create(&self, provider: String, facts: Vec<String>) -> Vec<Result<(), Box<dyn Error>>> {
        facts
            .into_iter()
            .map(|f| -> Result<(), Box<dyn Error>> {
                match self.connection.execute(
                    "INSERT INTO facts VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(fact) DO NOTHING ;",
                    [
                        (uuid::Uuid::new_v4().to_string()),
                        (f),
                        (provider.to_owned()),
                        (0.to_string()),
                        (Local::now().to_string()),
                    ],
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)?,
                }
            })
            .collect::<Vec<Result<(), Box<dyn Error>>>>()
    }

    pub fn get_random_fact(&self) -> Result<Option<(String, String)>, Box<dyn Error>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, fact FROM facts WHERE was_displayed = 0 ORDER BY created_at DESC LIMIT 1",
        )?;
        let mut rows = stmt.query([])?;
        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(Some((
                row.get::<usize, String>(0)?,
                row.get::<usize, String>(1)?,
            )))
        }
        Ok(results.pop().unwrap_or(None))
    }

    pub fn mark_as_read(&self, id: String) -> Result<(), Box<dyn Error>> {
        self.connection
            .execute("UPDATE facts SET was_displayed = 1 WHERE id = ?1", [(id)])?;
        Ok(())
    }
}
