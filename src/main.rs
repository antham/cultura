use std::process::exit;

use log::error;
use structopt::StructOpt;
use third_part::Crawler;

mod config;
mod daemon;
mod db;
mod fact;
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
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "fact", about = "Manage fact")]
    FactRoot(Fact),
    #[structopt(name = "daemon", about = "Manage the daemon harvesting facts")]
    DaemonRoot(Daemon),
    #[structopt(name = "init", about = "Generate the shell configuration")]
    InitRoot(Shell),
    #[structopt(name = "config", about = "Manage the configuration of the app")]
    ConfigRoot(Config),
    #[structopt(name = "doctor", about = "Troubleshoot issues with cultura")]
    DoctorRoot(Doctor),
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
    #[structopt(about = "Stop the daemon")]
    Stop {},
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
    #[structopt(about = "Define the template to use to display a fact")]
    SetTemplate { template: String },
    #[structopt(about = "Dump the current config")]
    Dump {},
    #[structopt(about = "Get the path of the config file")]
    GetConfigFilePath {},
}

#[derive(StructOpt, Debug)]
enum Doctor {
    #[structopt(about = "Stop the daemon and remove all cultura config and data")]
    Reset {},
    #[structopt(about = "Check if providers are working by performing a call with them")]
    RunProviders {},
}

fn main() {
    let a = Cultura::from_args();

    let config_resolver_result = config::ConfigResolver::new(home::home_dir());
    if config_resolver_result.is_err() {
        error!(
            "cannot bootstrap the config: {}",
            config_resolver_result.err().unwrap()
        );
        exit(0);
    }
    let config_resolver = config_resolver_result.unwrap();

    let third_part_services: Vec<Box<dyn Crawler>> = config_resolver.get_providers();

    let fact_repository_result = crate::db::Fact::new(&config_resolver.get_database_path());
    if fact_repository_result.is_err() {
        error!(
            "cannot bootstrap the fact repository: {}",
            fact_repository_result.err().unwrap()
        );
        exit(0);
    }
    let fact_repository = fact_repository_result.unwrap();
    let fact_service = fact::Fact::new(&config_resolver, &fact_repository, third_part_services);

    match a.command {
        Command::FactRoot(provider) => match provider {
            Fact::GenerateRandom {} => match fact_service.print_random() {
                Ok(_) => (),
                Err(e) => error!("an error occurred when printing fact: {}", e),
            },
        },
        Command::DaemonRoot(daemon) => match daemon {
            Daemon::Start {} => {
                match daemon::Daemon::new(&config_resolver, &fact_service).start() {
                    Ok(_) => (),
                    Err(e) => error!("cannot start daemon: {}", e),
                }
            }
            Daemon::Stop {} => match daemon::Daemon::new(&config_resolver, &fact_service).stop() {
                Ok(_) => println!("stop the daemon"),
                Err(e) => error!("cannot stop daemon: {}", e),
            },
        },
        Command::InitRoot(shell) => {
            let s = shell::Shell::new();
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
            Config::GetConfigFilePath {} => {
                println!(
                    "The config file path is located at {}",
                    config_resolver.get_config_file_path()
                )
            }
            Config::SetProviders { providers } => match config_resolver.set_providers(providers) {
                Ok(_) => {
                    println!("providers defined");
                }
                Err(e) => error!("cannot set the providers: {}", e),
            },
            Config::SetTemplate { template } => match config_resolver.set_template(template) {
                Ok(_) => {
                    println!("template defined");
                }
                Err(e) => error!("cannot set the template: {}", e),
            },
        },
        Command::DoctorRoot(doctor) => match doctor {
            Doctor::Reset {} => {
                match daemon::Daemon::new(&config_resolver, &fact_service).stop() {
                    Ok(_) => println!("* stop the daemon"),
                    Err(e) => error!("cannot stop daemon: {}", e),
                }
                match config_resolver.clear_all() {
                    Ok(_) => println!("* config folder deleted"),
                    Err(e) => error!("cannot remove the config folder: {}", e),
                }
            }
            Doctor::RunProviders {} => third_part::get_available_providers().iter().for_each(|i| {
                println!(
                    r#"Fetching provider {}
  Found {} facts
  Details
    {:?}
---
"#,
                    i.0,
                    i.1.get_facts().unwrap().len(),
                    i.1.get_facts().unwrap()
                )
            }),
        },
    }
}
