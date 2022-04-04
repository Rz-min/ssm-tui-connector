//
use super::ui::MenuItems;

use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}};
use anyhow::Result;
use tui::widgets::TableState;


pub struct App {
    pub select_menu: MenuItems,
    pub update_crypto_store_task: tokio::task::JoinHandle<()>,
    pub crypto_table_state: TableState,
}

impl App {
    pub fn new(running_flag: Arc<AtomicBool>) -> Result<App> {

        let update_crypto_store_task = tokio::spawn(async move {
            let clone_flag = Arc::clone(&running_flag);
        
            'outer: loop {
                if !clone_flag.load(Ordering::Relaxed) {
                    println!("get false signal and break crypto update task");
                    break 'outer;
                }


            }
        });

        Ok(App {
            select_menu: MenuItems::Home,
            update_crypto_store_task,
            crypto_table_state: TableState::default(),
        })
    }

    pub fn get_select_menu(&self) -> MenuItems {
        self.select_menu
    }

    pub fn get_crypto_ranking(&self) {
        
    }

    pub fn get_crypto_table_state(&self) -> TableState {
        self.crypto_table_state.clone()
    }
}
