//
use crate::inputs::EventHost;
use crate::app::App;
use anyhow::Result;
use std::io::{self, Stdout};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;
use tui::{
    backend::{Backend, TermionBackend},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use super::{draw_home, MenuItems, draw_crypto, draw_blockchain, draw_news, draw_stocks};

type ATerminal = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>;
pub struct Draw {
    pub terminal: Terminal<ATerminal>,
    pub app: App,
}

impl Draw {
    pub fn new(app: App) -> Result<Draw> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        Ok(Draw { terminal, app })
    }

    pub fn draw(&mut self, handler: &mut EventHost) -> Result<()> {
        self.terminal.draw(|mut f| {
            
            match handler.get_input() {
                termion::event::Key::Char('h') => {
                    self.app.select_menu = MenuItems::Home;
                },
                termion::event::Key::Char('c') => {
                    self.app.select_menu = MenuItems::Cryptocurrency;
                }
                termion::event::Key::Char('s') => {
                    self.app.select_menu = MenuItems::Stocks;
                }
                termion::event::Key::Char('n') => {
                    self.app.select_menu = MenuItems::News;
                }
                termion::event::Key::Char('b') => {
                    self.app.select_menu = MenuItems::BlockChain;
                }
                _ => {
                    self.app.select_menu;
                },
            }

            match self.app.select_menu {
                MenuItems::Home => {
                    draw_home(f, &mut self.app, handler);
                },
                MenuItems::Cryptocurrency => {
                    draw_crypto(f, &mut self.app, handler);
                },
                MenuItems::Stocks => {
                    draw_stocks(f, &mut self.app, handler);
                },
                MenuItems::News => {
                    draw_news(f, &mut self.app, handler);
                },
                MenuItems::BlockChain => {
                    draw_blockchain(f, &mut self.app, handler);
                },
            }

            

            // match handler.get_input() {
            //     termion::event::Key::Char('h') => {
            //         self.app.select_menu = MenuItems::Home;
            //     },
            //     termion::event::Key::Char('c') => {
            //         self.app.select_menu = MenuItems::Cryptocurrency;
            //     }
            //     termion::event::Key::Left => {
                    
            //     },
            //     termion::event::Key::Right => {

            //     },





            //     termion::event::Key::Backspace => todo!(),
            //     termion::event::Key::Up => todo!(),
            //     termion::event::Key::Down => todo!(),
            //     termion::event::Key::Home => todo!(),
            //     termion::event::Key::End => todo!(),
            //     termion::event::Key::PageUp => todo!(),
            //     termion::event::Key::PageDown => todo!(),
            //     termion::event::Key::BackTab => todo!(),
            //     termion::event::Key::Delete => todo!(),
            //     termion::event::Key::Insert => todo!(),
            //     termion::event::Key::F(_) => todo!(),
            //     termion::event::Key::Alt(_) => todo!(),
            //     termion::event::Key::Ctrl(_) => todo!(),
            //     termion::event::Key::Null => todo!(),
            //     termion::event::Key::Esc => todo!(),
            //     _ => todo!(),
            // }
        })?;

        Ok(())
    }
}
