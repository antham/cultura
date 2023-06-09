use log::error;

pub struct Logger {
    is_enabled: bool,
}

impl Logger {
    pub fn new(is_enabled: bool) -> Logger {
        Logger { is_enabled }
    }

    pub fn error<S: Into<String>>(&self, err: S) {
        if self.is_enabled {
            error!("{}", err.into());
        }
    }
}
