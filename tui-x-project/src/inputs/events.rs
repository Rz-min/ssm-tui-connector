//
use std::{io, sync::mpsc, thread, time::{Duration, Instant}};
use humantime::parse_duration;
use termion::event::Key;
use termion::input::TermRead;

#[derive(Debug, Clone, Copy)]
pub struct EventConfig {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl EventConfig {
    pub fn new(tick_rate: Duration) -> EventConfig {
        EventConfig { exit_key: Key::Ctrl('c'), tick_rate }
    }
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_secs(1),
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
    pub input_task: thread::JoinHandle<()>,
    pub tick_task: thread::JoinHandle<()>,
    pub last_input: Option<Key>,
}

impl EventHost {
    pub fn new(tick_rate: &Option<String>) -> EventHost {

        let config = match tick_rate {
            Some(v) => EventConfig::new(parse_duration(&v).unwrap()),
            None => EventConfig::default(),
        };

        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        let tick_tx = tx.clone();

        let input_task = thread::spawn(move || 'outer: loop {
            let stdin = io::stdin();

            for event in stdin.keys() {
                match event {
                    Ok(key) => {
                        if event_tx.send(Event::Input(key)).is_err() {
                            break 'outer;
                        }
                        match key {
                            Key::Char('q') => break 'outer,
                            Key::Esc => break 'outer,
                            _ => {}
                        }
                    }
                    Err(_e) => {
                        break 'outer;
                    }
                }
            }
        });

        let tick_task = thread::spawn(move || {
            let mut last_tick = Instant::now();
            'outer: loop {
                if last_tick.elapsed() >= config.tick_rate {
                    match tick_tx.send(Event::Tick) {
                        Ok(_) => last_tick = Instant::now(),
                        Err(_) => {
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
            tick_task,
            last_input: Some(Key::Char('h')),
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
            Event::Input(key) => match key {
                Key::Char('q') => {
                    println!("get recv: {:?}", &key);
                    self.last_input = Some(Key::Char('q'));
                    Signal::Finish
                }
                Key::Char('h') => {
                    self.last_input = Some(Key::Char('h'));
                    Signal::Other
                }
                Key::Char('c') => {
                    self.last_input = Some(Key::Char('c'));
                    Signal::Other
                }
                Key::Char('s') => {
                    self.last_input = Some(Key::Char('s'));
                    Signal::Other
                }
                Key::Char('n') => {
                    self.last_input = Some(Key::Char('n'));
                    Signal::Other
                }
                Key::Char('b') => {
                    self.last_input = Some(Key::Char('b'));
                    Signal::Other
                }
                Key::Left => {
                    self.last_input = Some(Key::Left);
                    Signal::Other
                },
                Key::Right => {
                    self.last_input = Some(Key::Right);
                    Signal::Other
                },
                Key::Up => {
                    self.last_input = Some(Key::Up);
                    Signal::Other
                },
                Key::Down => {
                    self.last_input = Some(Key::Down);
                    Signal::Other
                },
                Key::Backspace => todo!(),
                Key::Home => todo!(),
                Key::End => todo!(),
                Key::PageUp => todo!(),
                Key::PageDown => todo!(),
                Key::BackTab => todo!(),
                Key::Delete => todo!(),
                Key::Insert => todo!(),
                Key::F(_) => todo!(),
                Key::Alt(_) => todo!(),
                Key::Ctrl(_) => todo!(),
                Key::Null => todo!(),
                Key::Esc => todo!(),
                _ => {
                    self.last_input = None;
                    Signal::Other
                }
            },
            Event::Tick => Signal::Other,
        }
    }
}
