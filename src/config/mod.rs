use std::fs::DirBuilder;

const DATABASE_NAME: &str = "cultura.db";

#[derive(Clone)]
pub struct ConfigResolver {
    home_dir: String,
}

impl ConfigResolver {
    pub fn new() -> Result<ConfigResolver, ()> {
        match home::home_dir() {
            Some(path) => {
                let c = ConfigResolver {
                    home_dir: path.display().to_string(),
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
}
