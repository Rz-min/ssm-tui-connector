//
use std::{
    io, time::Duration, thread, sync::mpsc,
};
use termion::event::Key;
use termion::input::TermRead;

#[derive(Debug, Clone, Copy)]
pub struct EventConfig {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig { 
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

pub enum Signal {
    Finish,
    Other,
}

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct EventHost {
    rx: mpsc::Receiver<Event<Key>>,
    _tx: mpsc::Sender<Event<Key>>,
    pub input_task: std::thread::JoinHandle<()>,
    pub last_input: Option<Key>,
}

impl EventHost {
    pub fn new() -> EventHost {
        let _config = EventConfig::default();

        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        let input_task = thread::spawn(move ||{
            'outer: loop {
                let stdin = io::stdin();
                
                for event in stdin.keys() {
                    println!("event: {:?}", &event);
                    match event {
                        Ok(key) => {
                            if event_tx.send(Event::Input(key)).is_err() {
                                break;
                            }
                            match key {
                                Key::Char('q') => break 'outer,
                                Key::Esc => break 'outer,
                                _ => {},
                            }
                        }
                        Err(_e) => {
                            break 'outer;
                        }
                    }
                }
            }
        });

        EventHost { 
            rx,
            _tx: tx,
            input_task,
            last_input: Some(Key::Char('q')),
        }
    }

    pub fn get_input(&mut self) -> Key {
        match self.last_input {
            Some(v) => v,
            None => Key::Home,
        }
    }

    pub fn next(&mut self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn on_event(&mut self) -> Signal {
        match self.next().unwrap() {
            Event::Input(key) => {
                match key {
                    Key::Char('q') => {
                        println!("get recv: {:?}", &key);
                        self.last_input = Some(Key::Char('q'));
                        Signal::Finish
                    }
                    _ => {
                        self.last_input = None;
                        Signal::Other
                    },
                }
            }
            Event::Tick => Signal::Other,
        }
    }
}