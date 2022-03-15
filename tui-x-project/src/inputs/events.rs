use std::sync::atomic::AtomicBool;

//
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use termion::event::Event;
use termion::event::Key;

pub struct EventHost {
    rx: tokio::sync::mpsc::Receiver<Event>,
    _tx: tokio::sync::mpsc::Sender<Event>,
    stop_capture: Arc<AtomicBool>,
    command_input: Option<Key>,
}
impl EventHost {
    pub new() -> EventHost {

        EventHost {

        }
    }
}