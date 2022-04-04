//
mod vc;

pub use self::vc::VCManager;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CryptoCurrencyModel {
    cmc_rank: u64,
    name: String,
    symbol: String,
    circulating_supply: u64,
    total_supply: u64,
    market_cap_by_total_supply: u64,
    max_supply: u64,
    last_updated: String,
    quote: Vec<Quote>,
}

pub fn make(data_set: Vec<HashMap<String, Value>>) -> Vec<CryptoCurrencyModel> {
    let mut crypto_set = vec![];

    data_set.into_iter().for_each(|data| {
        let mut crypto = CryptoCurrencyModel::default();

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
                    crypto.circulating_supply = n.as_u64().unwrap_or(0);
                }
                _ => {}
            }
        }

        if let Some(res_total_supply) = data.get("total_supply") {
            match res_total_supply {
                Value::Number(n) => {
                    crypto.total_supply = n.as_u64().unwrap_or(0);
                }
                _ => {}
            }
        }

        if let Some(res_market_cap_by_total_supply) = data.get("market_cap_by_total_supply") {
            match res_market_cap_by_total_supply {
                Value::Number(n) => {
                    crypto.market_cap_by_total_supply = n.as_u64().unwrap_or(0);
                }
                _ => {}
            }
        }

        if let Some(res_max_supply) = data.get("max_supply") {
            match res_max_supply {
                Value::Number(n) => {
                    crypto.max_supply = n.as_u64().unwrap_or(0)
                }
                _ => {}
            }
        }
        
        if let Some(res_last_update) = data.get("last_update").map(|x| x.to_string()) {
            crypto.symbol = res_last_update;
        }

        crypto_set.push(crypto);
    });

    crypto_set
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
