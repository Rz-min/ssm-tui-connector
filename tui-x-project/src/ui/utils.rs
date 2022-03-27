//
use tui::style::{Style, Color, Modifier};
use tui::text::{Span, Spans};
use tui::widgets::{Tabs, Block, Borders};

use super::MenuItems;

pub fn menu_widgets<'a>(position: MenuItems) -> Tabs<'a> {
    let menu_items = vec!["Home", "Cryptocurrency", "Stocks", "News", "BlockChain"];

    let menu = menu_items.iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first, Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        }
    ).collect();

    let tabs = Tabs::new(menu)
        .select(position.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));
    tabs
}