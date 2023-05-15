use std::{error::Error, fs::File, thread, time::Duration};

use daemonize::Daemonize;

use crate::{config::ConfigResolver, fact::Fact};

pub struct Daemon<'a> {
    config_resolver: &'a ConfigResolver,
    fact: &'a Fact<'a>,
}

impl<'a> Daemon<'a> {
    pub fn new(
        config_resolver: &'a ConfigResolver,
        fact: &'a Fact,
    ) -> Result<Daemon<'a>, Box<dyn Error>> {
        File::create(config_resolver.get_stdout_file())?;
        File::create(config_resolver.get_stderr_file())?;

        Ok(Daemon {
            config_resolver,
            fact,
        })
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let stdout = File::open(self.config_resolver.get_stdout_file())?;
        let stderr = File::open(self.config_resolver.get_stderr_file())?;

        Daemonize::new()
            .pid_file(self.config_resolver.get_pid_file())
            .working_directory(self.config_resolver.get_working_dir())
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges")
            .start()?;

        loop {
            self.fact.update();
            thread::sleep(Duration::from_secs(
                60 * self.config_resolver.get_scheduler_interval_as_minutes(),
            ));
        }
    }
}
