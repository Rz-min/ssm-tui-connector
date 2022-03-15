//
use anyhow::Result;
use structopt::{clap, StructOpt};
use url::{Url, ParseError};

mod ui;
mod inputs;


#[derive(Debug, StructOpt)]
#[structopt(name = "tui x project")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: Url
}


#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
