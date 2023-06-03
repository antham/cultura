use std::{error::Error, fs::File, thread, time::Duration};

use daemonize::Daemonize;
use nix::{
    sys::signal::{kill, Signal},
    unistd::Pid,
};

const STDOUT: &str = "/dev/null";
const STDERR: &str = "/dev/null";
use crate::{config::ConfigResolver, fact::Fact};

pub struct Daemon<'a> {
    config_resolver: &'a ConfigResolver,
    fact: &'a Fact<'a>,
}

impl<'a> Daemon<'a> {
    pub fn new(config_resolver: &'a ConfigResolver, fact: &'a Fact) -> Daemon<'a> {
        Daemon {
            config_resolver,
            fact,
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let stdout = File::open(STDOUT)?;
        let stderr = File::open(STDERR)?;

        Daemonize::new()
            .pid_file(self.config_resolver.get_pid_file())
            .working_directory(self.config_resolver.get_working_dir())
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges")
            .start()?;

        loop {
            self.fact.update()?;
            thread::sleep(Duration::from_secs(
                60 * self.config_resolver.get_scheduler_interval_as_minutes(),
            ));
        }
    }

    pub fn stop(&self) -> Result<(), Box<dyn Error>> {
        let pid = Pid::from_raw(self.config_resolver.get_daemon_pid()?);
        match kill(pid, Signal::SIGKILL) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
