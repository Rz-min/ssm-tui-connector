//
mod app;
mod inputs;
mod ui;
mod utils;
mod vc;

use self::app::App;
use self::inputs::{EventHost, Signal};
use self::ui::Draw;
use self::utils::FiatCurrency;
use self::vc::VCManager;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use structopt::{clap, StructOpt};
use termion::event::Key;

#[derive(Debug, StructOpt, Serialize, Deserialize)]
#[structopt(name = "tui x project")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: String,
    #[structopt(short = "a", long = "api-key")]
    api_key: String,
    #[structopt(short = "c", long = "currency", possible_values(&FiatCurrency::variants()))]
    currency: FiatCurrency,
    #[structopt(short = "t", long = "update frequency")]
    update: Option<String>,
    #[structopt(short = "n", long = "update_cycle")]
    crypto_update_cycle: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let running_flag = Arc::new(AtomicBool::new(true));

    let mut handler = EventHost::new(&opt.update);
    let vc = VCManager::new(
        Arc::clone(&running_flag),
        opt.crypto_update_cycle.unwrap_or(10),
        &opt.api_key,
    );

    let app = App::new(vc).unwrap();

    let mut draw = Draw::new(app).unwrap();

    let crypto_subscribe = tokio::spawn(async move {});

    loop {
        match draw.draw(&mut handler) {
            Ok(_) => match handler.on_event() {
                Signal::Finish => match handler.get_input() {
                    Key::Char('q') => {
                        running_flag.store(false, Ordering::Relaxed);
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

    let (task,) = tokio::join!(crypto_subscribe);
    task?;

    handler.input_task.join().unwrap();

    Ok(())
}
