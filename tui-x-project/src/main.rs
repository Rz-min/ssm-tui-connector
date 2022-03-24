//
mod ui;
mod inputs;
mod app;
mod vc;

use self::vc::vc::VCManager;
use self::app::App;
use self::inputs::{EventHost, Signal};

use anyhow::Result;
use structopt::{clap, StructOpt};
use url::Url;
use termion::event::Key;

#[derive(Debug, StructOpt)]
#[structopt(name = "tui x project")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
struct Opt {
    #[structopt(short = "u", long = "url")]
    url: Url
}


#[tokio::main]
async fn main() -> Result<()> {

    let mut handler = EventHost::new();
    
    let vc = VCManager::new();

    let mut app = App::new().unwrap();


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
