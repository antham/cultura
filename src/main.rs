use std::process::exit;

use structopt::StructOpt;
use third_part::Crawler;

mod config;
mod daemon;
mod db;
mod fact;
mod logger;
mod shell;
mod third_part;

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
    #[structopt(short, long, env = "ENABLE_LOG")]
    enable_log: bool,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "fact", about = "Manage fact")]
    FactRoot(Fact),
    #[structopt(name = "daemon", about = "Run the daemon harvesting facts")]
    DaemonRoot(Daemon),
    #[structopt(name = "init", about = "Generate the shell configuration")]
    InitRoot(Shell),
    #[structopt(name = "config", about = "Manage the configuration of the app")]
    ConfigRoot(Config),
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

#[derive(StructOpt, Debug)]
enum Config {
    #[structopt(about = "Define the providers to enable")]
    SetProviders { providers: Vec<String> },
    #[structopt(about = "Dump the current config")]
    Dump {},
}

fn main() {
    let a = Cultura::from_args();

    let logger = logger::Logger::new(a.enable_log);

    let config_resolver_result = config::ConfigResolver::new(a.enable_log);
    if config_resolver_result.is_err() {
        logger.error(format!(
            "cannot bootstrap the config: {}",
            config_resolver_result.err().unwrap()
        ));
        exit(0);
    }
    let config_resolver = config_resolver_result.unwrap();

    let third_part_services: Vec<Box<dyn Crawler>> = third_part::get_available_providers()
        .into_iter()
        .filter(|(k, _)| match config_resolver.get_providers() {
            None => true,
            Some(providers) => providers.contains(k),
        })
        .map(|(_, v)| v)
        .collect::<Vec<Box<dyn Crawler>>>();

    let fact_repository_result = crate::db::Fact::new(&config_resolver.get_database_path());
    if fact_repository_result.is_err() {
        logger.error(format!(
            "cannot bootstrap the fact repository: {}",
            fact_repository_result.err().unwrap()
        ));
        exit(0);
    }
    let fact_repository = fact_repository_result.unwrap();
    let fact_service = fact::Fact::new(&logger, &fact_repository, third_part_services);

    match a.command {
        Command::FactRoot(provider) => match provider {
            Fact::GenerateRandom {} => {
                fact_service.print_random();
            }
        },
        Command::DaemonRoot(daemon) => match daemon {
            Daemon::Start {} => {
                match daemon::Daemon::new(&config_resolver, &logger, &fact_service) {
                    Ok(d) => d.start(),
                    Err(e) => logger.error(format!("cannot start daemon: {}", e)),
                }
            }
        },
        Command::InitRoot(shell) => {
            let s = shell::Shell::new(&config_resolver);
            match shell {
                Shell::Fish {} => s.generate_fish_config(),
                Shell::Bash {} => s.generate_bash_config(),
                Shell::Zsh {} => s.generate_zsh_config(),
            }
        }
        Command::ConfigRoot(conf) => match conf {
            Config::Dump {} => {
                println!("{}", config_resolver.get_config())
            }
            Config::SetProviders { providers } => {
                config_resolver.set_providers(providers).unwrap();
                println!("Option defined");
            }
        },
    }
}
