use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    error::Error,
    fmt::{self, Display},
    fs::{self, DirBuilder},
    path::PathBuf,
};

use crate::third_part::{self, Crawler};
const CONFIG_FILE_NAME: &str = "config.toml";
const DATABASE_NAME: &str = "cultura.db";
const DEFAULT_TEMPLATE: &str = r#"
__|>__:cyan $fact:yellow
"#;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    providers: Vec<Box<dyn Crawler>>,
    template: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"providers => {:?}
template  => {}"#,
            self.providers
                .iter()
                .map(|p| p.get_id())
                .collect::<Vec<String>>(),
            self.template,
        )
    }
}

#[derive(Clone, Default)]
pub struct ConfigResolver {
    home_dir: String,
    config: RefCell<Config>,
}

impl ConfigResolver {
    pub fn new(path: Option<PathBuf>) -> Result<ConfigResolver, Box<dyn Error>> {
        match path {
            Some(path) => {
                let mut c = ConfigResolver {
                    home_dir: path.display().to_string(),
                    ..ConfigResolver::default()
                };
                let config_file_path = c.resolve_relative_path(CONFIG_FILE_NAME);

                DirBuilder::new()
                    .recursive(true)
                    .create(c.get_root_config_path())?;

                if std::path::Path::new(&config_file_path).exists() {
                    let s = fs::read_to_string(c.resolve_relative_path(CONFIG_FILE_NAME))?;
                    c.config = RefCell::new(toml::from_str(s.as_str())?);
                    if c.config.borrow().providers.is_empty() {
                        c.config.borrow_mut().providers = third_part::get_available_providers()
                            .values()
                            .cloned()
                            .collect();
                    }
                } else {
                    let mut config = Config::default();
                    config.template = String::from(DEFAULT_TEMPLATE);
                    save_config(config.clone(), &c)?;
                    c.config = RefCell::new(config);
                }
                Ok(c)
            }
            None => Err("path cannot be found".to_string())?,
        }
    }

    fn get_root_config_path(&self) -> String {
        format!("{}/.config/cultura", self.home_dir)
    }

    pub fn resolve_relative_path(&self, path: &str) -> String {
        format!("{}/{}", self.get_root_config_path(), path)
    }

    pub fn get_config(&self) -> Config {
        self.config.borrow().clone()
    }

    pub fn get_config_file_path(&self) -> String {
        self.resolve_relative_path(CONFIG_FILE_NAME)
    }

    pub fn set_template(&self, template: String) -> Result<(), Box<dyn Error>> {
        self.config.borrow_mut().template = template;
        save_config(self.config.borrow().clone(), self)?;
        Ok(())
    }

    pub fn get_template(&self) -> String {
        self.config.borrow().template.clone()
    }

    pub fn set_providers(&self, providers: Vec<String>) -> Result<(), Box<dyn Error>> {
        let available_providers = third_part::get_available_providers();
        let ps = providers
            .clone()
            .into_iter()
            .filter(|p| available_providers.contains_key(p.as_str()))
            .map(|p| available_providers.get(p.as_str()).unwrap().clone())
            .collect::<Vec<Box<dyn third_part::Crawler>>>();

        if ps.len() != providers.len() {
            Err("some providers are invalid")?
        } else {
            self.config.borrow_mut().providers = ps;
            save_config(self.config.borrow().clone(), self)?;
            Ok(())
        }
    }

    pub fn get_providers(&self) -> Vec<Box<dyn Crawler>> {
        self.config.borrow().providers.clone()
    }

    pub fn get_database_path(&self) -> String {
        self.resolve_relative_path(DATABASE_NAME)
    }

    pub fn get_pid_file(&self) -> String {
        self.resolve_relative_path("cultura.pid")
    }

    pub fn get_daemon_pid(&self) -> Result<i32, Box<dyn Error>> {
        let pid_str = fs::read_to_string(self.get_pid_file())?;
        let pid = pid_str.trim().parse::<i32>()?;
        Ok(pid)
    }

    pub fn get_working_dir(&self) -> &str {
        "/tmp"
    }

    pub fn get_scheduler_interval_as_minutes(&self) -> u64 {
        5
    }

    pub fn clear_all(&self) -> Result<(), Box<dyn Error>> {
        fs::remove_dir_all(self.get_root_config_path())?;
        Ok(())
    }
}

fn save_config(config: Config, config_resolver: &ConfigResolver) -> Result<(), Box<dyn Error>> {
    let toml = toml::to_string(&config).unwrap();
    fs::write(
        config_resolver.resolve_relative_path(CONFIG_FILE_NAME),
        toml,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_config_resolver() {
        {
            let c = ConfigResolver::new(Some(tempdir().unwrap().into_path()));
            match c {
                Ok(config) => {
                    assert!(
                        Regex::new(r"^.*?/.config/cultura$")
                            .unwrap()
                            .is_match(&config.get_root_config_path()),
                        "root_config_path = {}",
                        &config.get_root_config_path(),
                    );
                    assert!(
                        Regex::new(r"^.*?/.config/cultura/cultura.db$")
                            .unwrap()
                            .is_match(&config.get_database_path()),
                        "database_path = {}",
                        &config.get_database_path(),
                    );
                    assert!(
                        Regex::new(r"^.*?/.config/cultura/cultura.pid$")
                            .unwrap()
                            .is_match(&config.get_pid_file()),
                        "pid_file = {}",
                        &config.get_database_path(),
                    );
                }
                Err(e) => panic!("{}", e),
            }
        }
    }

    #[test]
    fn test_accessors_providers() {
        let c = ConfigResolver::new(Some(tempdir().unwrap().into_path())).unwrap();
        match c.set_providers(vec!["whatever".to_string()]) {
            Err(e) => assert_eq!(e.to_string(), "some providers are invalid"),
            Ok(_) => panic!("must return an error"),
        };

        match c.set_providers(vec!["TIL".to_string()]) {
            Err(e) => panic!("must return no error: {}", e),
            Ok(_) => (),
        };
        println!("{}", c.get_config());
        assert_eq!(c.get_providers().len(), 1);
        assert_eq!(c.get_providers().first().unwrap().get_id(), "TIL");

        let c2 = ConfigResolver::new(Some(tempdir().unwrap().into_path())).unwrap();
        assert_eq!(c2.get_providers().len(), 0);
    }
}
