use structopt::StructOpt;

mod db;
mod reddit;
mod services;
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
    match a.command {
        Command::FactRoot(provider) => match provider {
            Fact::GenerateRandom {} => services::generate_random_fact(),
        },
        Command::DaemonRoot(daemon) => match daemon {
            Daemon::Start {} => services::start_daemon(),
        },
        Command::InitRoot(shell) => match shell {
            Shell::Fish {} => shell::generate_fish_config(),
            Shell::Bash {} => shell::generate_bash_config(),
            Shell::Zsh {} => shell::generate_zsh_config(),
        },
    }
}
