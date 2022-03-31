//
use super::ui::MenuItems;
use crate::vc::VCManager;
use anyhow::Result;

use tui::widgets::TableState;

pub struct App {
    pub vc: VCManager,
    pub select_menu: MenuItems,
    pub crypto_table_state: TableState,
}

impl App {
    pub fn new(vc: VCManager) -> Result<App> {
        Ok(App {
            vc,
            select_menu: MenuItems::Home,
            crypto_table_state: TableState::default(),
        })
    }

    pub fn get_select_menu(&self) -> MenuItems {
        self.select_menu
    }

    pub fn get_crypto_table_state(&self) -> TableState {
        self.crypto_table_state.clone()
    }
}
