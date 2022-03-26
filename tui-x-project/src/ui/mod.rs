//
mod ui;
pub mod home;

pub use self::home::draw_home;
pub use self::ui::Draw;
use std::convert::From;
use strum_macros::{EnumCount, EnumIter, EnumString, Display};

#[derive(Debug, Clone, Copy, Display, EnumCount, EnumIter, EnumString)]
pub enum MenuItems {
    #[strum(serialize = "Home")]
    Home,
    #[strum(serialize = "Cryptocurrency")]
    Cryptocurrency,
    #[strum(serialize = "Stocks")]
    Stocks,
    #[strum(serialize = "News")]
    News,
    #[strum(serialize = "BlockChain")]
    BlockChain,
}

impl From<MenuItems> for usize {
    fn from(i: MenuItems) -> Self {
        match i {
            MenuItems::Home => 0,
            MenuItems::Cryptocurrency => 1,
            MenuItems::Stocks => 2,
            MenuItems::News => 3,
            MenuItems::BlockChain => 4,
        }
    }
}