//
use anyhow::Result;
use structopt::{clap, StructOpt};
use url::{Url, ParseError};
use tokio::sync::mpsc;

mod ui;
mod inputs;

use self::ui::Draw;


#[derive(Debug, StructOpt)]
#[structopt(name = "tui x project")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: Url
}


#[tokio::main]
async fn main() -> Result<()> {

    let (data_send_tx, data_resv_rx) = mpsc::channel(1);

    let ui_manager = Draw::new()?ß;

    let crypt_task = tokio::spawn(async move {

    });

    'outer: loop {
        'inner: loop {
            ui_manager.draw();

            break 'inner;
        }

        break 'outer;
    }

    if let (Err(e), ) = tokio::join!(crypt_task) {
        println!("failed at join: {:?}", &e);
    }

    Ok(())
}
