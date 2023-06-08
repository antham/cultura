use std::error::Error;

use colored::Colorize;
use regex::Regex;

use crate::{config::ConfigResolver, db, third_part::Crawler};

const NO_FACT_MESSAGES: &str = "Stay tuned for more fascinating facts soon";

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
        println!("{}", self.generate_output(self.generate_random()?));
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

    fn generate_random(&self) -> Result<String, Box<dyn Error>> {
        let data = self.fact.get_random_fact()?;

        Ok(match data {
            Some((id, fact)) => {
                self.fact.mark_as_read(id)?;
                fact
            }
            None => NO_FACT_MESSAGES.to_string(),
        })
    }

    fn generate_output(&self, fact: String) -> String {
        let mut template = self.config_resolver.get_template();
        let t = template.clone();

        let data = Regex::new(r"(?:__(?:.+?)__|\$fact)(?::[a-z]+)*")
            .unwrap()
            .find_iter(t.as_str())
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();

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
            template = template.replace(d, format!("{}", text_formatted).as_str());
        }
        template
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Alphanumeric, Rng};
    use rusqlite::Connection;
    use serde::{Deserialize, Serialize};
    use tempfile::tempdir;

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

    fn generate_random_string(prefix: &str, suffix: &str) -> String {
        format!(
            "{}-{}{}",
            prefix,
            rand::thread_rng()
                .sample_iter(Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>(),
            suffix,
        )
    }

    #[test]
    fn test_update() {
        let database_name = generate_random_string("update", ".sqlite");

        let f = crate::db::Fact::new(database_name.as_str()).unwrap();
        let facts = vec![
            "whatever (whatever whatever) 1".to_string(),
            "whatever 2".to_string(),
        ];
        let third_part_services: Vec<Box<dyn Crawler>> = vec![Box::new(CrawlerMock { facts })];
        let config_resolver = ConfigResolver::new(Some(tempdir().unwrap().into_path())).unwrap();
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
        let database_name = &generate_random_string("generate_random", ".sqlite");

        let f = crate::db::Fact::new(database_name.as_str()).unwrap();
        f.create(
            "til".to_string(),
            vec!["fact1".to_string(), "fact2".to_string()],
        );
        let third_part_services = vec![];
        let config_resolver = ConfigResolver::new(Some(tempdir().unwrap().into_path())).unwrap();

        let fact = Fact::new(&config_resolver, &f, third_part_services);

        let mut facts: Vec<String> = vec![];

        let f1 = fact.generate_random();
        assert!(f1.is_ok());
        facts.push(f1.ok().unwrap());

        let f2 = fact.generate_random();
        assert!(f2.is_ok());
        facts.push(f2.ok().unwrap());

        let f3 = fact.generate_random();
        assert!(f3.is_ok());
        assert!(f3.ok().unwrap() == NO_FACT_MESSAGES);
    }

    #[test]
    fn test_generate_output() {
        let database_name = &generate_random_string("generate_output", ".sqlite");
        let f = crate::db::Fact::new(database_name.as_str()).unwrap();
        let third_part_services = vec![];
        let config_resolver = ConfigResolver::new(Some(tempdir().unwrap().into_path())).unwrap();

        {
            let fact = Fact::new(&config_resolver, &f, third_part_services.clone());
            let data = fact.generate_output("fact1".to_string());

            assert_eq!(data, "\n\u{1b}[36m|>\u{1b}[0m \u{1b}[33mfact1\u{1b}[0m\n");
        }
        {
            config_resolver
                .set_template(
                    r#"__Cultura__:magenta:bold

__|>__:cyan $fact:yellow"#
                        .to_string(),
                )
                .unwrap();
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
            let fact = Fact::new(&config_resolver, &f, third_part_services.clone());
            let data = fact.generate_output("fact1".to_string());
            assert_eq!(data, "\u{1b}[31mfact1\u{1b}[0m");
        }
        {
            config_resolver
                .set_template("__A text between space__:magenta:bold".to_string())
                .unwrap();
            let fact = Fact::new(&config_resolver, &f, third_part_services.clone());
            let data = fact.generate_output("fact1".to_string());
            assert_eq!(data, "\u{1b}[1;35mA text between space\u{1b}[0m");
        }
        {
            config_resolver
                .set_template("__ATextWithoutStyle__ __ATextWithStyles__:magenta".to_string())
                .unwrap();
            let fact = Fact::new(&config_resolver, &f, third_part_services.clone());
            let data = fact.generate_output("fact1".to_string());
            assert_eq!(data, "ATextWithoutStyle \u{1b}[35mATextWithStyles\u{1b}[0m");
        }
        {
            config_resolver
                .set_template("__A_text_with_dashes__:magenta".to_string())
                .unwrap();
            let fact = Fact::new(&config_resolver, &f, third_part_services.clone());
            let data = fact.generate_output("fact1".to_string());
            assert_eq!(data, "\u{1b}[35mA_text_with_dashes\u{1b}[0m");
        }
    }
}
