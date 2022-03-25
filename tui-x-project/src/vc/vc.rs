//
use anyhow::Result;
use std::{sync::{Arc, atomic::AtomicBool}, collections::BTreeMap};

pub struct Currency {

}

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