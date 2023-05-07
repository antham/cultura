use colored::Colorize;

use crate::{config, db::Fact};

pub fn print_random(log_enabled: bool) {
    let database_path = config::ConfigResolver::new(log_enabled)
        .unwrap()
        .get_database_path();
    let fact = crate::db::Fact::new(&database_path);
    let logger = crate::logger::Logger::new(log_enabled);

    match generate_random(fact) {
        Ok(Some(f)) => output(f),
        Ok(None) => logger.info("No result to resut"),
        Err(e) => logger.error(&e),
    }
}

fn generate_random(fact: Fact) -> Result<Option<String>, String> {
    match fact.get_random_fact() {
        Ok(Some((id, data))) => match fact.mark_as_read(id) {
            Ok(_) => Ok(Some(data)),
            Err(e) => Err(e.to_string()),
        },
        Ok(None) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

fn output(fact: String) {
    println!(
        r"{}

{} {}
",
        "Cultura".magenta().bold(),
        "|>".cyan(),
        fact.yellow(),
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_generate_random() {
        let database_name = "generate-random-fact";

        let _ = fs::remove_file(database_name);
        let f = crate::db::Fact::new(database_name);
        f.create(
            "til".to_string(),
            vec!["fact1".to_string(), "fact2".to_string()],
        );

        let mut facts: Vec<String> = vec![];

        let f1 = generate_random(crate::db::Fact::new(database_name));
        assert!(f1.is_ok());
        facts.push(f1.ok().unwrap().unwrap());

        let f2 = generate_random(crate::db::Fact::new(database_name));
        assert!(f2.is_ok());
        facts.push(f2.ok().unwrap().unwrap());

        let f3 = generate_random(crate::db::Fact::new(database_name));
        assert!(f3.is_ok());
        assert!(f3.ok().unwrap() == None);
    }
}
