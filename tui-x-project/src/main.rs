//
use anyhow::Result;
use structopt::{clap, StructOpt};
use url::Url;
use tokio::sync::mpsc;

mod ui;
mod inputs;
mod app;

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

    let (data_send_tx, data_recv_rx) = mpsc::channel(1);

    let ui_manager = Draw::new()?;


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
