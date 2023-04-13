use structopt::StructOpt;

mod db;
mod reddit;
mod wikipedia;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "cultura",
    about = "Improve your culture day by day",
    author = "Anthony HAMON <hamon.anth@gmail.com>",
    version = "0.0.1"
)]
struct Cultura {
    #[structopt(subcommand)]
    subcmd: Provider,
}

#[derive(StructOpt, Debug)]
enum Provider {
    #[structopt(about = "Get fact from the sub todayilearned on reddit")]
    TIL {},
    #[structopt(about = "Get fact from the do you know section of Wikipedia")]
    DYK {},
}

fn main() {
    let fact = db::Fact::new("/tmp/file.db");
    match reddit::get_til_facts() {
        Ok(v) => fact.create("til".to_string(), v),
        Err(e) => println!("{}", e),
    }
    match wikipedia::get_dyk_facts() {
        Ok(v) => fact.create("dyk".to_string(), v),
        Err(e) => println!("{}", e),
    }

    let a = Cultura::from_args();
    match a.subcmd {
        Provider::TIL {} => match fact.get_random_fact("til") {
            Some(s) => println!("{}", s),
            None => (),
        },
        Provider::DYK {} => match fact.get_random_fact("dyk") {
            Some(s) => println!("{}", s),
            None => (),
        },
    }
}
