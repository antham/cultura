use structopt::StructOpt;

mod reddit;

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
}

fn main() {
    let a = Cultura::from_args();
    match a.subcmd {
        Provider::TIL {} => match reddit::get_til_facts() {
            Ok(v) => println!("{}", v.first().unwrap()),
            Err(e) => println!("{}", e),
        },
    }
}
