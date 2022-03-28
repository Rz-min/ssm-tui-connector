//
use crate::app::App;
use crate::inputs::EventHost;
use super::utils::menu_widgets;
use tui::{Frame, backend::Backend};
use tui::layout::{Layout, Constraint, Direction};
use tui::widgets::{Block, Borders, Table, Cell, Row};
use tui::text::{Span, Spans};
use tui::style::{Color, Style, Modifier};

pub fn draw_crypto<B>(f: &mut Frame<B>, app: &mut App, _handler: &mut EventHost)
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

    let _crypto_table_state = app.get_crypto_table_state();
    let crypto_data = app.vc.get_crypto_ranking().unwrap(); //vector描画するvectorを貰う。

    let table = get_table(crypto_data);

    f.render_widget(table, chunks[2]);
}

pub fn get_table<'a>(data: Vec<Vec<String>>) -> Table<'a> {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = [
        "[r]ank", "[n]ame", "[p]rice", "[m]arket cap", "24 [v]olume", "[1]H%", 
        "[2]4H%", "[7]DH%", "[t]otal supply", "[a]vailable supply", "last [u]pdated"
        ]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    
    let rows = data.clone().into_iter().map(|item| {
        let height = item.iter().map(|content| content.chars().filter(|c| *c == '\n').count())
            .max().unwrap_or(0) + 1;
        let cells = data.iter().map(|_c| Cell::from(Spans::from(vec![
            Span::from("1"),
            Span::from("2"),
            Span::from("3"),
            Span::from("4"),
            Span::from("5"),
            Span::from("6"),
            Span::from("7"),
            Span::from("8"),
            Span::from("9"),
            Span::from("10"),
            Span::from("11"),
            Span::from("12"),

        ])));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });

    let table = Table::new(rows)
        .block(Block::default().borders(Borders::ALL).title("Cryptocurrency"))
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(5),
            Constraint::Percentage(10),
            Constraint::Percentage(7),
            Constraint::Percentage(10),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
            Constraint::Percentage(6),
        ])
        .style(Style::default().fg(Color::Green))
        .column_spacing(1);

    table
}