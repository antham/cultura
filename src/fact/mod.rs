use colored::Colorize;

use crate::config;

pub fn generate_random() {
    let config_resolver = config::ConfigResolver::new().unwrap();
    let database_path = &config_resolver.get_database_path();
    let fact = crate::db::Fact::new(&database_path);

    match fact.get_random_fact() {
        Some((id, data)) => {
            match fact.mark_as_read(id) {
                _ => (),
            };
            println!(
                r"{}

{} {}
",
                "Cultura".magenta().bold(),
                "|>".cyan(),
                data.yellow()
            )
        }
        None => (),
    }
}
