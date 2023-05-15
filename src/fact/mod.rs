use std::error::Error;

use colored::Colorize;
use regex::Regex;

use crate::{db, third_part::Crawler};

pub struct Fact<'a> {
    fact: &'a db::Fact,
    third_part_services: Vec<Box<dyn Crawler>>,
}

impl<'a> Fact<'a> {
    pub fn new(fact: &'a db::Fact, third_part_services: Vec<Box<dyn Crawler>>) -> Self {
        Fact {
            fact,
            third_part_services,
        }
    }

    pub fn print_random(&self) -> Result<(), Box<dyn Error>> {
        self.generate_random()?.map(|fact| {
            self.output(fact);
        });
        Ok(())
    }

    pub fn update(&self) -> Result<(), Box<dyn Error>> {
        let r: Result<(), Box<dyn Error>> = Ok(());

        self.third_part_services
            .iter()
            .map(|service: &Box<dyn Crawler>| -> Result<(), Box<dyn Error>> {
                let parens = Regex::new("\\(.+\\)").unwrap();
                let multi_space = Regex::new(r"\s+").unwrap();
                let facts = service
                    .get_facts()?
                    .iter()
                    .map(|s| parens.replace_all(s, "").to_string())
                    .map(|s| multi_space.replace_all(s.as_str(), " ").to_string())
                    .collect::<Vec<String>>();

                let r: Result<(), Box<dyn Error>> = Ok(());

                self.fact
                    .create(service.get_id(), facts)
                    .iter()
                    .fold(r, |acc, item| match item {
                        Ok(_) => acc,
                        Err(e) => match acc {
                            Ok(_) => Err(e.to_string().into()),
                            Err(e_acc) => Err(format!("{}, {}", e_acc, e).into()),
                        },
                    })
            })
            .fold(r, |acc, item| match item {
                Ok(_) => acc,
                Err(e) => match acc {
                    Ok(_) => Err(e.to_string().into()),
                    Err(e_acc) => Err(format!("{}, {}", e_acc, e).into()),
                },
            })
    }

    fn generate_random(&self) -> Result<Option<String>, Box<dyn Error>> {
        let data = self.fact.get_random_fact()?;

        Ok(match data {
            Some((id, fact)) => {
                self.fact.mark_as_read(id)?;
                Some(fact)
            }
            None => None,
        })
    }

    fn output(&self, fact: String) {
        println!(
            r"{}

{} {}
",
            "Cultura".magenta().bold(),
            "|>".cyan(),
            fact.yellow(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rusqlite::Connection;

    use crate::logger;

    use super::*;

    struct CrawlerMock {
        facts: Vec<String>,
    }

    impl Crawler for CrawlerMock {
        fn get_facts(&self) -> Result<Vec<String>, String> {
            Ok(self.facts.to_owned())
        }

        fn get_id(&self) -> String {
            "crawlermock".to_string()
        }
    }

    #[test]
    fn test_update() {
        let database_name = "update-fact";

        let _ = fs::remove_file(database_name);
        let f = crate::db::Fact::new(database_name).unwrap();
        let facts = vec![
            "whatever (whatever whatever) 1".to_string(),
            "whatever 2".to_string(),
        ];
        let logger = logger::Logger::new(false);
        let third_part_services: Vec<Box<dyn Crawler>> = vec![Box::new(CrawlerMock { facts })];
        let fact = Fact::new(&logger, &f, third_part_services);
        fact.update();

        let conn = Connection::open(database_name).unwrap();
        let mut stmt = conn.prepare("SELECT * FROM facts").unwrap();
        let mut rows = stmt.query([]).unwrap();
        let row1 = rows.next().unwrap().unwrap();
        assert_eq!("whatever 1", row1.get_unwrap::<usize, String>(1));
        assert_eq!("crawlermock", row1.get_unwrap::<usize, String>(2));

        let row2 = rows.next().unwrap().unwrap();
        assert_eq!("whatever 2", row2.get_unwrap::<usize, String>(1));
        assert_eq!("crawlermock", row2.get_unwrap::<usize, String>(2));
    }

    #[test]
    fn test_generate_random() {
        let database_name = "generate-random-fact";

        let _ = fs::remove_file(database_name);
        let f = crate::db::Fact::new(database_name).unwrap();
        f.create(
            "til".to_string(),
            vec!["fact1".to_string(), "fact2".to_string()],
        );
        let logger = logger::Logger::new(false);
        let third_part_services = vec![];

        let fact = Fact::new(&logger, &f, third_part_services);

        let mut facts: Vec<String> = vec![];

        let f1 = fact.generate_random();
        assert!(f1.is_ok());
        facts.push(f1.ok().unwrap().unwrap());

        let f2 = fact.generate_random();
        assert!(f2.is_ok());
        facts.push(f2.ok().unwrap().unwrap());

        let f3 = fact.generate_random();
        assert!(f3.is_ok());
        assert!(f3.ok().unwrap() == None);
    }
}
