//
use anyhow::Result;
use structopt::{clap, StructOpt};
use url::Url;
use tokio::sync::mpsc;

mod ui;
mod inputs;
mod app;

use self::app::App;


#[derive(Debug, StructOpt)]
#[structopt(name = "tui x project")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: Url
}


#[tokio::main]
async fn main() -> Result<()> {

    //let (data_send_tx, data_recv_rx) = mpsc::channel(1);

    let mut app = App::new().unwrap();


    loop {
        app.draw();
        
        break;
    }


    Ok(())
}
