//
use crate::vc::VCManager;
use super::ui::{MenuItems};
use anyhow::Result;

pub struct App {
    pub vc: VCManager,
    pub select_menu: MenuItems,
}

impl App {
    pub fn new(vc: VCManager) -> Result<App> {

        Ok(App {
            vc,
            select_menu: MenuItems::Home,
        })
    }

    pub fn get_select_menu(&self) -> MenuItems {
        self.select_menu
    }
}
