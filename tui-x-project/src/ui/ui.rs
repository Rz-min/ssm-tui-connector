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

use super::{draw_home, MenuItems};

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
            
            match self.app.select_menu {
                MenuItems::Home => {
                    draw_home(f, &mut self.app, handler);
                },
                MenuItems::Cryptocurrency => todo!(),
                MenuItems::Stocks => todo!(),
                MenuItems::News => todo!(),
                MenuItems::BlockChain => todo!(),
            }
            // match handler.get_input() {
            //     termion::event::Key::Char('h') => {
            //         draw_home(f, &self.app);
            //     },
            //     termion::event::Key::Left => {
            //         self.app.select_menu = MenuItems::BlockChain;
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
