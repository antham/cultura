use std::{fs::File, thread, time::Duration};

use daemonize::Daemonize;

use crate::{
    config::{self, ConfigResolver},
    db::Fact,
    logger::{self, Logger},
};

pub struct Daemon<'a> {
    daemonize: Daemonize<&'a str>,
    config_resolver: ConfigResolver,
    logger: Logger,
    fact: Fact,
}

impl<'a> Daemon<'a> {
    pub fn new(enable_log: bool) -> Daemon<'a> {
        let config_resolver = config::ConfigResolver::new(enable_log).unwrap();
        let logger = logger::Logger::new(enable_log);
        let fact = crate::db::Fact::new(&config_resolver.get_database_path()).unwrap();

        let stdout = File::create(config_resolver.get_stdout_file()).unwrap();
        let stderr = File::create(config_resolver.get_stderr_file()).unwrap();

        let daemonize = Daemonize::new()
            .pid_file(config_resolver.get_pid_file())
            .working_directory(config_resolver.get_working_dir())
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges");
        Daemon {
            daemonize,
            config_resolver,
            logger,
            fact,
        }
    }

    pub fn start(self) {
        match self.daemonize.start() {
            Ok(_) => loop {
                let v: Vec<(&str, fn() -> Result<Vec<String>, String>)> = vec![
                    ("til", crate::reddit::get_til_facts),
                    ("dyk", crate::wikipedia::get_dyk_facts),
                ];
                v.iter().for_each(|(id, f)| -> () {
                    match f() {
                        Ok(v) => {
                            self.fact
                                .create(id.to_string(), v)
                                .iter()
                                .for_each(|val| match val {
                                    Ok(_) => (),
                                    Err(e) => self.logger.error(&e),
                                });
                            ()
                        }
                        Err(e) => self.logger.error(&e),
                    }
                });
                thread::sleep(Duration::from_secs(
                    60 * self.config_resolver.get_scheduler_interval_as_minutes(),
                ));
            },
            Err(e) => self.logger.error(&e),
        }
    }
}
