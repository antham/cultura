use std::{fs::File, thread, time::Duration};

use clokwerk::{Scheduler, TimeUnits};
use daemonize::Daemonize;

use crate::config::{self, ConfigResolver};

pub struct Daemon<'a> {
    daemonize: Daemonize<&'a str>,
    config_resolver: ConfigResolver,
}

impl<'a> Daemon<'a> {
    pub fn new() -> Daemon<'a> {
        let config_resolver = config::ConfigResolver::new().unwrap();

        let stdout = File::create("/dev/null").unwrap();
        let stderr = File::create("/dev/null").unwrap();

        let daemonize = Daemonize::new()
            .pid_file(
                config_resolver
                    .resolve_relative_path("cultura.pid")
                    .as_str(),
            )
            .working_directory("/tmp")
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges");
        Daemon {
            daemonize,
            config_resolver,
        }
    }

    pub fn start(self) {
        let mut scheduler = Scheduler::new();

        let database_path = self.config_resolver.get_database_path();

        scheduler
            .every(10.seconds())
            .run(move || update_facts(database_path.to_owned()));

        match self.daemonize.start() {
            Ok(_) => {
                update_facts(self.config_resolver.get_database_path());
                loop {
                    scheduler.run_pending();
                    thread::sleep(Duration::from_secs(60 * 5));
                }
            }
            Err(_) => (),
        }
    }
}

fn update_facts(database_path: String) {
    let fact = crate::db::Fact::new(&database_path);
    match crate::reddit::get_til_facts() {
        Ok(v) => fact.create("til".to_string(), v),
        Err(e) => println!("{}", e),
    }
    match crate::wikipedia::get_dyk_facts() {
        Ok(v) => fact.create("dyk".to_string(), v),
        Err(e) => println!("{}", e),
    }
}
