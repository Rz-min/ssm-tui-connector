//
mod ui;
mod inputs;
mod app;
mod vc;

use self::vc::vc::VCManager;
use self::app::App;
use self::inputs::EventHost;

use anyhow::Result;
use structopt::{clap, StructOpt};
use url::Url;
use tokio::sync::mpsc;

#[derive(Debug, StructOpt)]
#[structopt(name = "tui x project")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: Url
}


#[tokio::main]
async fn main() -> Result<()> {

    let (handle_tx, handle_rx) = mpsc::channel(1);

    let handler = EventHost::new();
    
    let vc = VCManager::new();

    let mut app = App::new().unwrap();


    loop {
        app.draw();
        
        break;
    }


    Ok(())
}
