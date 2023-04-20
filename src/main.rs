use clokwerk::{Scheduler, TimeUnits};
use daemonize::Daemonize;
use std::{fs::File, thread, time::Duration};
use structopt::StructOpt;

mod db;
mod reddit;
mod wikipedia;

const DATABASE_NAME: &str = "/tmp/file.db";

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
    let fact = db::Fact::new(DATABASE_NAME);
    let a = Cultura::from_args();
    match a.command {
        Command::FactRoot(provider) => match provider {
            Fact::GenerateRandom {} => match fact.get_random_fact() {
                Some((id, data)) => {
                    fact.mark_as_read(id);
                    println!("{}", data)
                }
                None => (),
            },
        },
        Command::DaemonRoot(daemon) => match daemon {
            Daemon::Start {} => {
                let stdout = File::create("/dev/null").unwrap();
                let stderr = File::create("/dev/null").unwrap();

                let daemonize = Daemonize::new()
                    .pid_file("/tmp/cultura.pid")
                    .working_directory("/tmp")
                    .stdout(stdout)
                    .stderr(stderr)
                    .privileged_action(|| "Executed before drop privileges");

                let mut scheduler = Scheduler::new();
                scheduler.every(10.seconds()).run(|| update_facts());

                match daemonize.start() {
                    Ok(_) => {
                        update_facts();
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

fn update_facts() {
    let fact = db::Fact::new(DATABASE_NAME);
    match reddit::get_til_facts() {
        Ok(v) => fact.create("til".to_string(), v),
        Err(e) => println!("{}", e),
    }
    match wikipedia::get_dyk_facts() {
        Ok(v) => fact.create("dyk".to_string(), v),
        Err(e) => println!("{}", e),
    }
}
