//
mod ui;
mod inputs;
mod app;
mod vc;
mod utils;

use self::vc::VCManager;
use self::app::App;
use self::inputs::{EventHost, Signal};
use self::utils::FiatCurrency;

use anyhow::Result;
use structopt::{clap, StructOpt};
use url::Url;
use std::sync::{Arc, atomic::AtomicBool};
use termion::event::Key;

#[derive(Debug, StructOpt)]
#[structopt(name = "tui x project")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: Url,
    #[structopt(short = "c", long = "currency", possible_values(&FiatCurrency::variants()))]
    currency: FiatCurrency,
}


#[tokio::main]
async fn main() -> Result<()> {

    let mut running_flag = Arc::new(AtomicBool::new(false));
    let mut running_clone = running_flag.clone();

    let mut handler = EventHost::new();
    
    let vc = VCManager::new(running_clone);

    let mut app = App::new(vc).unwrap();


    loop {
        match app.draw() {
            Ok(v) => {
                match handler.on_event() {
                    Signal::Finish => {
                        match handler.get_input() {
                            Key::Char('q') => {
                                break;
                            }
                            _ => continue,
                        }
                    }
                    Signal::Other => continue,
                }
            }
            Err(e) => {

            }
        }
        
    }

    handler.input_task.join().unwrap();

    Ok(())
}
