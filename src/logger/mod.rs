use log::{error, info};

pub struct Logger {
    is_enabled: bool,
}

impl Logger {
    pub fn new(is_enabled: bool) -> Logger {
        Logger { is_enabled }
    }

    pub fn info(&self, log: &str) {
        if self.is_enabled {
            info!("{}", log);
        }
    }

    pub fn error(&self, err: &dyn ToString) {
        if self.is_enabled {
            error!("{}", err.to_string());
        }
    }
}
