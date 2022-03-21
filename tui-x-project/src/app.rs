//
use anyhow::Result;
use tui::{backend::{TermionBackend, Backend}, Frame, widgets::{Block, BorderType, Borders}};
use crate::inputs::EventHost;
use tui::Terminal;
use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;
use termion::raw::{RawTerminal, IntoRawMode};
use std::io::{self, Stdout};

type ATerminal = TermionBackend<RawTerminal<Stdout>>;
struct App<B>
where
    B: Backend
{
    terminal: Terminal<B>
}

impl<B> App<B>
where
    B: Backend,
{
    pub fn new() -> Result<App<ATerminal>> {
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        Ok(App { 
            terminal
        })
    }

    pub fn draw(&mut self, handler: EventHost) -> Result<()> {

        self.terminal.draw(|mut f | {

            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);

            f.render_widget(block, f.size());
        })?;

        Ok(())
    }
}