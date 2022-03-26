//
use anyhow::Result;
use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicBool, Arc},
};

pub struct Currency {}

pub struct VCManager {
    running_flag: Arc<AtomicBool>,
    crypto_store: BTreeMap<String, Currency>,
}

impl VCManager {
    pub fn new(running_flag: Arc<AtomicBool>) -> VCManager {
        VCManager {
            running_flag,
            crypto_store: BTreeMap::new(),
        }
    }
}
