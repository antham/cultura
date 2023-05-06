use std::fs::DirBuilder;

const DATABASE_NAME: &str = "cultura.db";

#[derive(Clone)]
pub struct ConfigResolver {
    home_dir: String,
    enable_log: bool,
}

impl ConfigResolver {
    pub fn new(enable_debug: bool) -> Result<ConfigResolver, ()> {
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
                    Err(_) => Err(()),
                }
            }
            None => Err(()),
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
}
