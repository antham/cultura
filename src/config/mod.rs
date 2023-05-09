use std::fs::DirBuilder;

const DATABASE_NAME: &str = "cultura.db";

#[derive(Clone)]
pub struct ConfigResolver {
    home_dir: String,
    enable_log: bool,
}

impl ConfigResolver {
    pub fn new(enable_debug: bool) -> Result<ConfigResolver, String> {
        match home::home_dir() {
            Some(path) => {
                let c = ConfigResolver {
                    home_dir: path.display().to_string(),
                    enable_log: enable_debug,
                };
                match DirBuilder::new()
                    .recursive(true)
                    .create(c.get_root_config_path())
                {
                    Ok(_) => Ok(c),
                    Err(e) => Err(format!("cannot create config file : {}", e)),
                }
            }
            None => Err("user home path cannot be found".to_string()),
        }
    }

    fn get_root_config_path(&self) -> String {
        format!("{}/.config/cultura", self.home_dir)
    }

    pub fn resolve_relative_path(&self, path: &str) -> String {
        format!("{}/{}", self.get_root_config_path(), path)
    }

    pub fn get_database_path(&self) -> String {
        self.resolve_relative_path(DATABASE_NAME)
    }

    pub fn get_pid_file(&self) -> &str {
        "cultura.pid"
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
}
