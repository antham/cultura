use std::error::Error;

use colored::Colorize;
use regex::Regex;

use crate::{config::ConfigResolver, db, third_part::Crawler};

pub struct Fact<'a> {
    config_resolver: &'a ConfigResolver,
    fact: &'a db::Fact,
    third_part_services: Vec<Box<dyn Crawler>>,
}

impl<'a> Fact<'a> {
    pub fn new(
        config_resolver: &'a ConfigResolver,
        fact: &'a db::Fact,
        third_part_services: Vec<Box<dyn Crawler>>,
    ) -> Self {
        Fact {
            config_resolver,
            fact,
            third_part_services,
        }
    }

    pub fn print_random(&self) -> Result<(), Box<dyn Error>> {
        self.generate_random()?.map(|fact| {
            println!("{}", self.generate_output(fact));
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

    fn generate_output(&self, fact: String) -> String {
        let mut template = self.config_resolver.get_template();
        let mut acc = String::new();
        let mut data: Vec<String> = vec![];
        let mut start_acc = false;
        for (i, c) in template.to_string().chars().enumerate() {
            if c == '_' || c == '$' {
                start_acc = true;
            }
            if Regex::new(r"\s").unwrap().is_match(&c.to_string()) {
                start_acc = false;
            }
            if start_acc {
                acc.push(c);
            }
            if (i + 1 == template.len() || !start_acc) && acc.len() > 0 {
                data.push(acc.to_owned());
                acc.clear();
            }
        }

        for d in data {
            let items = d.split(":").collect::<Vec<&str>>();
            let mut text_formatted = items
                .get(0)
                .unwrap()
                .replace("__", "")
                .replace("$fact", fact.as_str())
                .to_string()
                .normal();
            for text_format in items.into_iter().skip(1) {
                text_formatted = match text_format {
                    "blue" => text_formatted.blue(),
                    "red" => text_formatted.red(),
                    "green" => text_formatted.green(),
                    "black" => text_formatted.black(),
                    "yellow" => text_formatted.yellow(),
                    "white" => text_formatted.white(),
                    "purple" => text_formatted.purple(),
                    "cyan" => text_formatted.cyan(),
                    "magenta" => text_formatted.magenta(),
                    "bold" => text_formatted.bold(),
                    "dimmed" => text_formatted.dimmed(),
                    "italic" => text_formatted.italic(),
                    "underline" => text_formatted.underline(),
                    _ => text_formatted.normal(),
                };
            }
            template = template.replace(d.as_str(), format!("{}", text_formatted).as_str());
        }
        template
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rusqlite::Connection;
    use serde::{Deserialize, Serialize};
    use tempfile::tempdir;

    use super::*;

    #[derive(Serialize, Deserialize, Clone)]
    struct CrawlerMock {
        facts: Vec<String>,
    }

    #[typetag::serde]
    impl Crawler for CrawlerMock {
        fn get_facts(&self) -> Result<Vec<String>, Box<dyn Error>> {
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
        let third_part_services: Vec<Box<dyn Crawler>> = vec![Box::new(CrawlerMock { facts })];
        let config_resolver =
            ConfigResolver::new(Some(tempdir().unwrap().into_path()), false).unwrap();
        let fact = Fact::new(&config_resolver, &f, third_part_services);
        fact.update().unwrap();

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
        let third_part_services = vec![];
        let config_resolver =
            ConfigResolver::new(Some(tempdir().unwrap().into_path()), false).unwrap();

        let fact = Fact::new(&config_resolver, &f, third_part_services);

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

    #[test]
    fn test_generate_output() {
        let database_name = "generate-random-fact";
        let _ = fs::remove_file(database_name);
        let f = crate::db::Fact::new(database_name).unwrap();
        let third_part_services = vec![];
        let config_resolver =
            ConfigResolver::new(Some(tempdir().unwrap().into_path()), false).unwrap();

        {
            let fact = Fact::new(&config_resolver, &f, third_part_services.clone());
            let data = fact.generate_output("fact1".to_string());

            assert_eq!(
                data,
                "\u{1b}[1;35mCultura\u{1b}[0m\n\n".to_owned()
                    + "\u{1b}[36m|>\u{1b}[0m \u{1b}[33mfact1\u{1b}[0m"
            );
        }
        {
            config_resolver
                .set_template("$fact:red".to_string())
                .unwrap();
            let fact = Fact::new(&config_resolver, &f, third_part_services);
            let data = fact.generate_output("fact1".to_string());
            assert_eq!(data, "\u{1b}[31mfact1\u{1b}[0m");
        }
    }
}
