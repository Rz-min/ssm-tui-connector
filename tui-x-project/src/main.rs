//
mod app;
mod inputs;
mod ui;
mod utils;
mod vc;

use self::app::App;
use self::ui::Draw;
use self::inputs::{EventHost, Signal};
use self::utils::FiatCurrency;
use self::vc::VCManager;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicBool, Arc};
use structopt::{clap, StructOpt};
use termion::event::Key;

#[derive(Debug, StructOpt, Serialize, Deserialize)]
#[structopt(name = "tui x project")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: String,
    #[structopt(short = "c", long = "currency", possible_values(&FiatCurrency::variants()))]
    currency: FiatCurrency,
    #[structopt(short = "t", long = "update frequency")]
    update: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let mut running_flag = Arc::new(AtomicBool::new(false));
    let mut running_clone = running_flag.clone();

    let mut handler = EventHost::new(&opt.update);
    let vc = VCManager::new(running_clone);
    let app = App::new(vc).unwrap();

    let mut draw = Draw::new(app).unwrap();

    loop {
        match draw.draw(&mut handler) {
            Ok(_) => match handler.on_event() {
                Signal::Finish => match handler.get_input() {
                    Key::Char('q') => {
                        break;
                    }
                    _ => continue,
                },
                Signal::Other => continue,
            },
            Err(e) => {
                println!("Couldn't draw: {}", e);
                break;
            }
        }
    }

    handler.input_task.join().unwrap();

    Ok(())
}
