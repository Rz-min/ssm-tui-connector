//
use anyhow::Result;
use reqwest::header::{self, HeaderValue, ACCEPT};
use reqwest::Client;
use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::{
    sync::mpsc::{self, Receiver},
    time::{sleep, Duration},
};

pub struct Currency {
    pub name: String,
    pub price: String,
}

pub struct VCManager {
    crypto_store: BTreeMap<String, Vec<Currency>>,
    rx: Receiver<u64>,
}

impl VCManager {
    pub fn new(running_flag: Arc<AtomicBool>, crypto_update: u64, api_key: &str) -> VCManager {
        let (tx, rx) = mpsc::channel(1);

        let task = tokio::spawn(async move {
            let clone_flag = Arc::clone(&running_flag);

            let url = format!("");

            let mut headers = header::HeaderMap::new();
            headers.contains_key("X-CMC_PRO_API_KEY");

            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_static("src"));

            let client = Client::builder()
                .default_headers(headers)
                .build()
                .expect("Couldn't set up request client");

            'outer: loop {
                if !clone_flag.load(Ordering::Relaxed) {
                    break 'outer;
                }

                let client = client.clone();
                let res = client.get(url.clone()).send().await.expect("");

                if let Err(_) = tx.send(1).await {
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

    pub fn update_crypto_store(&mut self) {
        while let Ok(message) = self.rx.try_recv() {
            print!("message: {:?}", message);
        }
    }
}
