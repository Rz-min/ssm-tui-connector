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

use super::draw_home;

type ATerminal = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>;
pub struct Draw {
    pub terminal: Terminal<ATerminal>,
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

        Ok(Draw { terminal })
    }

    pub fn draw(&mut self, handler: &mut EventHost) -> Result<()> {
        self.terminal.draw(|mut f| {
            draw_home(f)
        })?;

        Ok(())
    }
}
