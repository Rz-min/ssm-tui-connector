//
use crate::app::App;
use crate::inputs::EventHost;
use super::utils::menu_widgets;
use tui::{Frame, backend::Backend};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::style::{Style, Color};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, BorderType};

pub fn draw_home<B>(f: &mut Frame<B>, app: &mut App, _handler: &mut EventHost)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(1),
                Constraint::Percentage(90),
            ]
            .as_ref(),
        )
        .split(f.size());

    let menu = menu_widgets(app.get_select_menu());

    f.render_widget(menu, chunks[0]);

    let middle = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access pets, 'a' to add random new pets and 'd' to delete the currently selected pet.")]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green))
        .title("Home")
        .border_type(BorderType::Plain)
    );

    f.render_widget(middle, chunks[2]);
}