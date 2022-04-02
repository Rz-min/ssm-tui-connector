//
mod vc;

pub use self::vc::VCManager;

use std::{collections::HashMap, ops::Index};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CryptoCurrencyModel {
    cmc_rank: u64,
    name: String,
    symbol: String,
    num_market_pairs: u64,
    circulating_supply: u64,
    total_supply: u64,
    market_cap_by_total_supply: u64,
    max_supply: u64,
    last_updated: String,
    date_added: String,
    quote: Vec<Quote>,
}

impl CryptoCurrencyModel {
    fn make(data_set: Vec<HashMap<String, Value>>) -> CryptoCurrencyModel {
        let mut crypto = CryptoCurrencyModel::default();

        data_set.into_iter().for_each(|data| {
            if let Some(res_rank) = data.get("cmc_rank") {
                match res_rank {
                    Value::Number(n) => {
                        crypto.cmc_rank = n.as_u64().unwrap().clone();
                    },
                    _ => {}
                }
            }

            if let Some(res_name) = data.get("name").map(|x| x.to_string()) {
                crypto.name = res_name;
            }

            if let Some(res_symbol) = data.get("symbol").map(|x| x.to_string()) {
                crypto.symbol = res_symbol;
            }

            if let Some(res_circulating_supply) = data.get("circulating_supply") {
                match res_circulating_supply {
                    Value::Number(n) => {
                        crypto.circulating_supply = n.as_u64().unwrap().clone();
                    }
                    _ => {}
                }
            }

        });

        crypto
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    price: u32,
    volume_24h: u32,
    volume_change_24h: f64,
    percent_change_1h: f64,
    percent_change_24h: f64,
    percent_change_7d: f64,
    market_cap: f64,
    market_cap_dominance: u32,
    fully_diluted_market_cap: f64,
    last_updated: String,
}
