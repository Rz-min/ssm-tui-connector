//
use crate::app::App;
use crate::inputs::EventHost;
use super::utils::menu_widgets;
use tui::{Frame, backend::Backend};
use tui::layout::{Layout, Constraint, Direction};

pub fn draw_blockchain<B>(f: &mut Frame<B>, app: &mut App, _handler: &mut EventHost)
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

}