//
use anyhow::Result;
use tui::{backend::{TermionBackend, Backend}, Frame};
use crate::inputs::EventHost;
use tui::Terminal;
use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;
use termion::raw::{RawTerminal, IntoRawMode};
use std::io::{self, Stdout};

type TTerminal = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>;
pub struct Draw<B>
where
    B: Backend,
{
    terminal: Terminal<B>,
}

impl<B> Draw<B>
where
    B: Backend,
{
    fn new() -> Result<Draw<TTerminal>> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(Draw { 
            terminal
        })
    }

    fn draw(&mut self, handler: EventHost) -> Result<()> {

        Ok(())
    }
}