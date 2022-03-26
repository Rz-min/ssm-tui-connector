//
use crate::app::App;
use super::MenuItems;
use tui::{Frame, backend::Backend};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color, Modifier};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Tabs, Borders};

pub fn draw_home<B>(f: &mut Frame<B>, )
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

    let menu_items = vec!["Home", "Cryptocurrency", "Stocks", "News", "BlockChain"];

    let menu = menu_items.iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first, Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        }
    ).collect();

    let tabs = Tabs::new(menu)
        .select(MenuItems::Home.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));

    f.render_widget(tabs, chunks[0]);
}