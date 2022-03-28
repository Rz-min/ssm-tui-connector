//
use anyhow::Result;
use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicBool, Arc},
};

pub struct Currency {
    pub name: String,
    pub price: String,
}

pub struct VCManager {
    running_flag: Arc<AtomicBool>,
    crypto_store: BTreeMap<String, Vec<Currency>>,
}

impl VCManager {
    pub fn new(running_flag: Arc<AtomicBool>) -> VCManager {
        VCManager {
            running_flag,
            crypto_store: BTreeMap::new(),
        }
    }

    pub fn get_crypto_ranking(&self) -> Result<Vec<Vec<String>>> {
        let data = vec![];
        
        Ok(data)
    }
}
