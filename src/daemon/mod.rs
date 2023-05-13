use std::{fs::File, thread, time::Duration};

use daemonize::Daemonize;

use crate::{config::ConfigResolver, fact::Fact, logger::Logger};

pub struct Daemon<'a> {
    config_resolver: &'a ConfigResolver,
    logger: &'a Logger,
    fact: &'a Fact<'a>,
}

impl<'a> Daemon<'a> {
    pub fn new(
        config_resolver: &'a ConfigResolver,
        logger: &'a Logger,
        fact: &'a Fact,
    ) -> Result<Daemon<'a>, String> {
        let stdout_r = File::create(config_resolver.get_stdout_file());
        let stderr_r = File::create(config_resolver.get_stderr_file());

        match (stdout_r, stderr_r) {
            (Ok(_), Ok(_)) => Ok(Daemon {
                config_resolver,
                logger,
                fact,
            }),
            (Err(e), Ok(_)) => Err(format!("cannot open stdout file: {}", e)),
            (Ok(_), Err(e)) => Err(format!("cannot open stderr file: {}", e)),
            (Err(out_e), Err(err_e)) => Err(format!(
                "cannot open stdout and stderr files: {} {}",
                out_e, err_e
            )),
        }
    }

    pub fn start(&self) {
        let stdout = File::open(self.config_resolver.get_stdout_file()).unwrap();
        let stderr = File::open(self.config_resolver.get_stderr_file()).unwrap();

        let d = Daemonize::new()
            .pid_file(self.config_resolver.get_pid_file())
            .working_directory(self.config_resolver.get_working_dir())
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges");

        match d.start() {
            Ok(_) => loop {
                self.fact.update();
                thread::sleep(Duration::from_secs(
                    60 * self.config_resolver.get_scheduler_interval_as_minutes(),
                ));
            },
            Err(e) => self.logger.error(e.to_string()),
        }
    }
}
