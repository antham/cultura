use std::{fs::File, thread, time::Duration};

use daemonize::Daemonize;

use crate::{config::ConfigResolver, db::Fact, logger::Logger};

pub struct Daemon<'a> {
    daemonize: Daemonize<&'a str>,
    config_resolver: &'a ConfigResolver,
    logger: &'a Logger,
    fact: &'a Fact,
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
            (Ok(stdout), Ok(stderr)) => {
                let daemonize = Daemonize::new()
                    .pid_file(config_resolver.get_pid_file())
                    .working_directory(config_resolver.get_working_dir())
                    .stdout(stdout)
                    .stderr(stderr)
                    .privileged_action(|| "Executed before drop privileges");
                Ok(Daemon {
                    daemonize,
                    config_resolver,
                    logger,
                    fact,
                })
            }
            (Err(e), Ok(_)) => Err(format!("cannot open stdout file: {}", e)),
            (Ok(_), Err(e)) => Err(format!("cannot open stderr file: {}", e)),
            (Err(out_e), Err(err_e)) => Err(format!(
                "cannot open stdout and stderr files: {} {}",
                out_e, err_e
            )),
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
                                    Err(e) => self.logger.error(e.to_string()),
                                });
                            ()
                        }
                        Err(e) => self.logger.error(e.to_string()),
                    }
                });
                thread::sleep(Duration::from_secs(
                    60 * self.config_resolver.get_scheduler_interval_as_minutes(),
                ));
            },
            Err(e) => self.logger.error(e.to_string()),
        }
    }
}
