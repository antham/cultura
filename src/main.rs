use config::ConfigResolver;
use structopt::StructOpt;

mod config;
mod daemon;
mod db;
mod fact;
mod reddit;
mod shell;
mod wikipedia;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "cultura",
    about = "Improve your culture day after day",
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
    #[structopt(name = "init", about = "Generate the shell configuration")]
    InitRoot(Shell),
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

#[derive(StructOpt, Debug)]
enum Shell {
    #[structopt(about = "Generate a fish config")]
    Fish {},
    #[structopt(about = "Generate a bash config")]
    Bash {},
    #[structopt(about = "Generate a zsh config")]
    Zsh {},
}

fn main() {
    let a = Cultura::from_args();
    let database_path = ConfigResolver::new().unwrap().get_database_path();

    match a.command {
        Command::FactRoot(provider) => match provider {
            Fact::GenerateRandom {} => {
                fact::generate_random(database_path);
                ()
            }
        },
        Command::DaemonRoot(daemon) => match daemon {
            Daemon::Start {} => {
                daemon::Daemon::new().start();
            }
        },
        Command::InitRoot(shell) => match shell {
            Shell::Fish {} => shell::generate_fish_config(),
            Shell::Bash {} => shell::generate_bash_config(),
            Shell::Zsh {} => shell::generate_zsh_config(),
        },
    }
}
