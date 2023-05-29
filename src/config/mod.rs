use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{self, Display},
    fs::{self, DirBuilder},
};

use crate::third_part::{self, Crawler};
const DATABASE_NAME: &str = "cultura.db";

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    providers: Vec<Box<dyn Crawler>>,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "providers => {:?}",
            self.providers
                .iter()
                .map(|p| p.get_id())
                .collect::<Vec<String>>(),
        )
    }
}

#[derive(Clone, Default)]
pub struct ConfigResolver {
    home_dir: String,
    enable_log: bool,
    config: Config,
}

impl ConfigResolver {
    pub fn new(enable_debug: bool) -> Result<ConfigResolver, Box<dyn Error>> {
        match home::home_dir() {
            Some(path) => {
                let mut c = ConfigResolver {
                    home_dir: path.display().to_string(),
                    enable_log: enable_debug,
                    ..ConfigResolver::default()
                };
                let config_file_path = c.resolve_relative_path("config.toml");

                DirBuilder::new()
                    .recursive(true)
                    .create(c.get_root_config_path())?;

                if std::path::Path::new(&config_file_path).exists() {
                    let s = fs::read_to_string(c.resolve_relative_path("config.toml"))?;
                    c.config = toml::from_str(s.as_str())?;
                } else {
                    let mut config = Config::default();
                    config.providers = third_part::get_available_providers()
                        .values()
                        .cloned()
                        .collect();
                    save_config(config, &c)?
                }
                Ok(c)
            }
            None => Err("user home path cannot be found".to_string())?,
        }
    }

    fn get_root_config_path(&self) -> String {
        format!("{}/.config/cultura", self.home_dir)
    }

    pub fn resolve_relative_path(&self, path: &str) -> String {
        format!("{}/{}", self.get_root_config_path(), path)
    }

    pub fn get_config(&self) -> &Config {
        &self.config
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
            Err("Some providers are invalid")?
        } else {
            let mut c = self.config.clone();
            c.providers = ps;
            save_config(c, self)?;
            Ok(())
        }
    }

    pub fn get_providers(&self) -> Vec<Box<dyn Crawler>> {
        self.config.providers.clone()
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

    pub fn get_stdout_file(&self) -> String {
        if self.enable_log {
            self.resolve_relative_path("stdout.log").to_string()
        } else {
            "/dev/null".to_string()
        }
    }

    pub fn get_stderr_file(&self) -> String {
        if self.enable_log {
            self.resolve_relative_path("stderr.log").to_string()
        } else {
            "/dev/null".to_string()
        }
    }

    pub fn get_scheduler_interval_as_minutes(&self) -> u64 {
        5
    }

    pub fn is_log_enabled(&self) -> bool {
        self.enable_log
    }
}

fn save_config(config: Config, config_resolver: &ConfigResolver) -> Result<(), Box<dyn Error>> {
    let toml = toml::to_string(&config).unwrap();
    fs::write(config_resolver.resolve_relative_path("config.toml"), toml)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_config_resolver() {
        {
            let c = ConfigResolver::new(false);
            match c {
                Ok(config) => {
                    assert!(config.enable_log == false);
                    assert!(
                        Regex::new(r"^/home/.*?/.config/cultura$")
                            .unwrap()
                            .is_match(&config.get_root_config_path()),
                        "root_config_path = {}",
                        &config.get_root_config_path(),
                    );
                    assert!(
                        Regex::new(r"^/home/.*?/.config/cultura/cultura.db$")
                            .unwrap()
                            .is_match(&config.get_database_path()),
                        "database_path = {}",
                        &config.get_database_path(),
                    );
                    assert!(
                        Regex::new(r"^/home/.*?/.config/cultura/cultura.pid$")
                            .unwrap()
                            .is_match(&config.get_pid_file()),
                        "pid_file = {}",
                        &config.get_database_path(),
                    );
                    assert_eq!(&config.get_stdout_file(), "/dev/null");
                    assert_eq!(&config.get_stderr_file(), "/dev/null");
                }
                Err(e) => panic!("{}", e),
            }
        }

        {
            let c = ConfigResolver::new(true);
            match c {
                Ok(config) => {
                    assert!(config.enable_log == true);
                    assert!(
                        Regex::new(r"^/home/.*?/.config/cultura/stdout.log$")
                            .unwrap()
                            .is_match(&config.get_stdout_file()),
                        "stdout_file = {}",
                        &config.get_stdout_file(),
                    );
                    assert!(
                        Regex::new(r"^/home/.*?/.config/cultura/stderr.log$")
                            .unwrap()
                            .is_match(&config.get_stderr_file()),
                        "stderr_file = {}",
                        &config.get_stderr_file(),
                    );
                }
                Err(e) => panic!("{}", e),
            }
        }
    }

    #[test]
    fn test_accessors_providers() {
        let c = ConfigResolver::new(false).unwrap();
        match c.set_providers(vec!["whatever".to_string()]) {
            Err(e) => assert_eq!(e.to_string(), "Some providers are invalid"),
            Ok(_) => panic!("must return an error"),
        };

        match c.set_providers(vec!["til".to_string()]) {
            Err(e) => panic!("must return no error: {}", e),
            Ok(_) => (),
        };

        let c2 = ConfigResolver::new(false).unwrap();
        assert_eq!(c2.get_providers().len(), 1);
        assert_eq!(c2.get_providers().first().unwrap().get_id(), "til");
    }
}
