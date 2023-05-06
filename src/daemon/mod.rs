use std::{fs::File, thread, time::Duration};

use daemonize::Daemonize;

use crate::config::{self, ConfigResolver};

const PID_FILE: &str = "cultura.pid";
const WORKING_DIRECTORY: &str = "/tmp";
const STDOUT_FILE: &str = "/dev/null";
const STDERR_FILE: &str = "/dev/null";
const SCHEDULER_INTERVAL_AS_MINUTES: u64 = 5;

pub struct Daemon<'a> {
    daemonize: Daemonize<&'a str>,
    config_resolver: ConfigResolver,
}

impl<'a> Daemon<'a> {
    pub fn new() -> Daemon<'a> {
        let config_resolver = config::ConfigResolver::new().unwrap();

        let stdout = File::create(STDOUT_FILE).unwrap();
        let stderr = File::create(STDERR_FILE).unwrap();

        let daemonize = Daemonize::new()
            .pid_file(config_resolver.resolve_relative_path(PID_FILE).as_str())
            .working_directory(WORKING_DIRECTORY)
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges");
        Daemon {
            daemonize,
            config_resolver,
        }
    }

    pub fn start(self) {
        match self.daemonize.start() {
            Ok(_) => loop {
                let fact = crate::db::Fact::new(&self.config_resolver.get_database_path());

                let v: Vec<(&str, fn() -> Result<Vec<String>, String>)> = vec![
                    ("til", crate::reddit::get_til_facts),
                    ("dyk", crate::wikipedia::get_dyk_facts),
                ];
                v.iter().for_each(|(id, f)| -> () {
                    match f() {
                        Ok(v) => {
                            fact.create(id.to_string(), v);
                            ()
                        }
                        Err(e) => println!("{}", e),
                    }
                });
                thread::sleep(Duration::from_secs(60 * SCHEDULER_INTERVAL_AS_MINUTES));
            },
            Err(_) => (),
        }
    }
}
