use clokwerk::{Scheduler, TimeUnits};
use colored::Colorize;
use daemonize::Daemonize;
use std::{
    fs::{DirBuilder, File},
    thread,
    time::Duration,
};
use structopt::StructOpt;

mod db;
mod reddit;
mod wikipedia;

const DATABASE_NAME: &str = "cultura.db";

#[derive(StructOpt, Debug)]
#[structopt(
    name = "cultura",
    about = "Improve your culture day by day",
    author = "Anthony HAMON <hamon.anth@gmail.com>",
    version = "0.0.1"
)]
struct Cultura {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "fact", about = "Manage fact")]
    FactRoot(Fact),
    #[structopt(name = "daemon", about = "Run the daemon harvesting facts")]
    DaemonRoot(Daemon),
}

#[derive(StructOpt, Debug)]
enum Fact {
    #[structopt(about = "Generate a random fact")]
    GenerateRandom {},
}

#[derive(StructOpt, Debug)]
enum Daemon {
    #[structopt(about = "Start the daemon")]
    Start {},
}

fn main() {
    let config_resolver = ConfigResolver::new().unwrap();
    let database_path = config_resolver.resolve_relative_path(DATABASE_NAME);
    let fact = db::Fact::new(&database_path);
    let a = Cultura::from_args();
    match a.command {
        Command::FactRoot(provider) => match provider {
            Fact::GenerateRandom {} => match fact.get_random_fact() {
                Some((id, data)) => {
                    match fact.mark_as_read(id) {
                        _ => (),
                    };
                    println!(
                        r"{}

{} {}
",
                        "Cultura".magenta().bold(),
                        "|>".cyan(),
                        data.yellow()
                    )
                }
                None => (),
            },
        },
        Command::DaemonRoot(daemon) => match daemon {
            Daemon::Start {} => {
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

                let d = database_path.clone();
                let mut scheduler = Scheduler::new();
                scheduler
                    .every(10.seconds())
                    .run(move || update_facts(d.as_str()));

                match daemonize.start() {
                    Ok(_) => {
                        update_facts(&database_path);
                        loop {
                            scheduler.run_pending();
                            thread::sleep(Duration::from_secs(60 * 5));
                        }
                    }
                    Err(_) => (),
                }
            }
        },
    }
}

fn update_facts(database_path: &str) {
    let fact = db::Fact::new(database_path);
    match reddit::get_til_facts() {
        Ok(v) => fact.create("til".to_string(), v),
        Err(e) => println!("{}", e),
    }
    match wikipedia::get_dyk_facts() {
        Ok(v) => fact.create("dyk".to_string(), v),
        Err(e) => println!("{}", e),
    }
}

#[derive(Clone)]
struct ConfigResolver {
    home_dir: String,
}

impl ConfigResolver {
    fn new() -> Result<ConfigResolver, ()> {
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

    fn resolve_relative_path(&self, path: &str) -> String {
        format!("{}/{}", self.get_root_config_path(), path)
    }
}
