//
use super::utils::menu_widgets;
use crate::app::App;
use crate::inputs::EventHost;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Row, Table};
use tui::{backend::Backend, Frame};
use termion::event::Key;

#[derive(Debug, Clone)]
pub struct CryptoPrint {
    pub cmc_rank: String,
    pub name: String,
    pub symbol: String,
    pub circulating_supply: String,
    pub total_supply: String,
    pub market_cap_by_total_supply: String,
    pub max_supply: String,
}

impl CryptoPrint {
    pub fn new(
        cmc_rank: String,
        name: String,
        symbol: String,
        circulating_supply: String,
        total_supply: String,
        market_cap_by_total_supply: String,
        max_supply: String,
    ) -> CryptoPrint {
        CryptoPrint { 
            cmc_rank,
            name,
            symbol,
            circulating_supply,
            total_supply,
            market_cap_by_total_supply,
            max_supply,
        }
    }
    
}

pub fn draw_crypto<B>(f: &mut Frame<B>, app: &mut App, handler: &mut EventHost)
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

    let crypto_data = app.get_crypto_ranking();

    let data_length = crypto_data.len();

    match handler.get_input() {
        Key::Right => {
            app.table_state_next(data_length);
            handler.last_input = None;
        },
        Key::Left => {
            app.table_state_previous(data_length);
            handler.last_input = None;
        },
        _ => {},
    }

    let crypto_table_state = app.get_crypto_table_state();

    let table = get_table(crypto_data);

    f.render_stateful_widget(table, chunks[2], &mut app.crypto_table_state);
}

pub fn get_table<'a>(data: Vec<CryptoPrint>) -> Table<'a> {
    let normal_style = Style::default().bg(Color::Blue);

    let header_cells = [
        "[r]ank",
        "[n]ame",
        "[p]rice",
        "[m]arket cap",
        "24 [v]olume",
        "[1]H%",
        "[2]4H%",
        "[7]DH%",
        "[t]otal supply",
        "[a]vailable supply",
        "last [u]pdated",
    ]
    .iter()
    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));

    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let rows = data.into_iter().map(|crypto| {
        let cells = Cell::from(Spans::from(vec![
            Span::from(crypto.cmc_rank),
            Span::from(crypto.name),
            Span::from(crypto.symbol),
            Span::from(crypto.circulating_supply),
            Span::from(crypto.total_supply),
            Span::from(crypto.market_cap_by_total_supply),
            Span::from(crypto.max_supply),
        ]));

        Row::new(cells)
    });

    let table = Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cryptocurrency"),
        )
        .header(header)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED).fg(Color::Red))
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
