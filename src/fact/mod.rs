use colored::Colorize;
use regex::Regex;

use crate::{db, logger::Logger};

pub struct Fact<'a> {
    logger: &'a Logger,
    fact: &'a db::Fact,
}

impl<'a> Fact<'a> {
    pub fn new(logger: &'a Logger, fact: &'a db::Fact) -> Self {
        Fact { logger, fact }
    }

    pub fn print_random(&self) {
        match self.generate_random() {
            Ok(Some(f)) => self.output(f),
            Ok(None) => self.logger.info("No result to resut"),
            Err(e) => self.logger.error(&e),
        }
    }

    pub fn update(&self) {
        let v: Vec<(&str, fn() -> Result<Vec<String>, String>)> = vec![
            ("til", crate::reddit::get_til_facts),
            ("dyk", crate::wikipedia::get_dyk_facts),
        ];
        v.iter().for_each(|(id, f)| -> () {
            match f() {
                Ok(v) => {
                    let parens = Regex::new("\\(.+\\)").unwrap();
                    let multi_space = Regex::new(r"\s+").unwrap();

                    self.fact
                        .create(
                            id.to_string(),
                            v.iter()
                                .map(|s| parens.replace_all(s.as_str(), "").to_string())
                                .map(|s| multi_space.replace_all(s.as_str(), " ").to_string())
                                .collect::<Vec<String>>(),
                        )
                        .iter()
                        .for_each(|val| match val {
                            Ok(_) => (),
                            Err(e) => self.logger.error(e.to_string()),
                        });
                    ()
                }
                Err(e) => self.logger.error(e.to_string()),
            }
        });
    }

    fn generate_random(&self) -> Result<Option<String>, String> {
        match self.fact.get_random_fact() {
            Ok(Some((id, data))) => match self.fact.mark_as_read(id) {
                Ok(_) => Ok(Some(data)),
                Err(e) => Err(e.to_string()),
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
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

    use crate::logger;

    use super::*;

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

        let fact = Fact::new(&logger, &f);

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
