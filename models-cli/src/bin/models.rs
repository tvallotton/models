use clap::{crate_version, App, ArgMatches};
use console::style;
use dotenv::dotenv;
use sqlx_models_cli::Opt;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let matches = Opt::clap().version(crate_version!()).get_matches();
    
    // no special handling here
    if let Err(error) = sqlx_models_cli::run(Opt::from_arg_matches(&matches)).await {
        println!("{} {}", style("error:").bold().red(), error);
        std::process::exit(1);
    }
}
