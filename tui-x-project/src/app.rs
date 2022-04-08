//
use super::ui::MenuItems;
use crate::ui::CryptoPrint;

use tokio::sync::{mpsc::Receiver};
use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}};
use anyhow::Result;
use tui::widgets::TableState;


pub struct App {
    pub select_menu: MenuItems,
    pub update_crypto_store_task: tokio::task::JoinHandle<()>,
    pub crypto_rx: Receiver<Vec<CryptoPrint>>,
    pub crypto_store: Vec<CryptoPrint>,
    pub crypto_table_state: TableState,
}

impl App {
    pub fn new(
        running_flag: Arc<AtomicBool>,
        crypto_receive: Receiver<Vec<CryptoPrint>>,
    ) -> Result<App> {

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
            crypto_rx: crypto_receive,
            crypto_store: vec![],
            crypto_table_state: TableState::default(),
        })
    }

    pub fn get_select_menu(&self) -> MenuItems {
        self.select_menu
    }

    pub fn get_crypto_ranking(&mut self) -> Vec<CryptoPrint> {
        match self.crypto_rx.try_recv() {
            Ok(data_set) => {
                self.crypto_store.clear();
                self.crypto_store = data_set.clone();
                data_set
            },
            Err(_) => self.crypto_store.clone(),
        }
    }

    pub fn get_crypto_table_state(&self) -> TableState {
        self.crypto_table_state.clone()
    }

    pub fn table_state_previous(&mut self, len: usize) {
        let position = match self.crypto_table_state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.crypto_table_state.select(Some(position));
    }

    pub fn table_state_next(&mut self, len: usize) {
        let position = match self.crypto_table_state.selected() {
            Some(i) => {
                if i >= (len - 1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.crypto_table_state.select(Some(position));
    }

}
