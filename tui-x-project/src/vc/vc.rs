//
use anyhow::Result;
use std::{
    collections::BTreeMap,
    sync::{atomic::{AtomicBool, Ordering}, Arc}
};
use tokio::{sync::mpsc::{self, Receiver}, time::{sleep, Duration}};

pub struct Currency {
    pub name: String,
    pub price: String,
}

pub struct VCManager {
    crypto_store: BTreeMap<String, Vec<Currency>>,
    rx: Receiver<u64>,
}

impl VCManager {
    pub fn new(running_flag: Arc<AtomicBool>, crypto_update: u64) -> VCManager {

        let (tx, rx) = mpsc::channel(1);

        let task = tokio::spawn(async move {
            let clone_flag = Arc::clone(&running_flag);

            'outer: loop {
                if !clone_flag.load(Ordering::Relaxed) {
                    break 'outer;
                }

                //here http request to api server

                if let Err(e) = tx.send(1).await {
                    println!("send error in crypto(VCManager)");
                    break 'outer;
                }

                sleep(Duration::from_secs(crypto_update)).await;
            }
        });

        VCManager {
            crypto_store: BTreeMap::new(),
            rx,
        }
    }

    pub fn get_crypto_ranking(&self) -> Result<Vec<Vec<String>>> {
        let data = vec![];
        
        Ok(data)
    }

    pub async fn update_crypto_store() {
        //try_recv()
    }
}
