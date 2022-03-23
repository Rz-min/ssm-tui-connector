use std::sync::atomic::AtomicBool;
//
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::io;
use std::time::Duration;
use std::thread;
use termion::event::Event;
use termion::event::Key;
use tokio::sync::mpsc;

pub struct EventHost {
    rx: tokio::sync::mpsc::Receiver<Event>,
    tx: tokio::sync::mpsc::Sender<Event>,
    input_task: std::thread::JoinHandle<()>,
    stop_capture: Arc<AtomicBool>,
    command_input: Option<Key>,
}

impl EventHost {
    pub fn new() -> EventHost {

        let (tx, rx) = mpsc::channel(1);

        let stop_capture = Arc::new(AtomicBool::new(false));

        let input_task = thread::spawn(move || {
            loop {
                let stdin = io::stdin();

                for event in stdin.keys() {
                    match event {
                        Ok(key) => {
                            if tx.send(Event::Input(key)).is_err() {
                                return;
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            
        });

        EventHost { 
            rx, 
            tx,
            input_task,
            stop_capture, 
            command_input,
        }
    }
}