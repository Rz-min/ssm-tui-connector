//
use crate::app::App;
use crate::inputs::EventHost;
use super::utils::menu_widgets;
use tui::{Frame, backend::Backend};
use tui::layout::{Layout, Constraint, Direction, Alignment};
use tui::widgets::{Block, Paragraph, Borders, BorderType};
use tui::text::{Span, Spans};
use tui::style::{Color, Style};

pub fn draw_news<B>(f: &mut Frame<B>, app: &mut App, _handler: &mut EventHost)
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
        Spans::from(vec![Span::raw("Under")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Construction")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("")]),
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