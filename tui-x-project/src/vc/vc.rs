//
use super::{CryptoCurrencyModel, make};
use anyhow::{Result, bail};
use reqwest::header::{self, HeaderValue, ACCEPT};
use reqwest::{Client, StatusCode};
use serde::{Serialize, Deserialize};
use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    time::{sleep, Duration},
};
use std::collections::HashMap;
use serde_json::Value;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub data: Vec<HashMap<String, Value>>,
}

pub struct VCManager {
    pub running_flag: Arc<AtomicBool>,
    pub task: tokio::task::JoinHandle<()>,
    crypto_store: BTreeMap<String, Vec<CryptoCurrencyModel>>,
    rx: Receiver<Vec<CryptoCurrencyModel>>,
    crypto_data_send_tx: Sender<i32>,
}

impl VCManager {
    pub fn new(
        running_flag: Arc<AtomicBool>,
        crypto_update: u64,
        api_key: &'static str,
        url: String,
        crypto_data_send_tx: Sender<i32>,
    ) -> VCManager {
        let (tx, rx) = mpsc::channel(1);

        let clone_flag = Arc::clone(&running_flag);

        let task = tokio::spawn(async move {

            let mut url = url.clone();
            let api_key = api_key.clone();

            let url = if url.ends_with("/") {
                url.pop().unwrap();
                url
            } else {
                url
            };

            let f = format!("{}/v1/cryptocurrency/listings/latest", url);
            let url = Url::parse(&f).unwrap();

            let mut headers = header::HeaderMap::new();
            headers.contains_key("X-CMC_PRO_API_KEY");
            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            headers.insert("X-CMC_PRO_API_KEY", HeaderValue::from_static(api_key));

            let client = Client::builder()
                .default_headers(headers)
                .build()
                .expect("Couldn't set up request client");

            'outer: loop {
                if !clone_flag.load(Ordering::Relaxed) {
                    println!("get false and break vc subscribe");
                    break 'outer;
                }

                let client = client.clone();
                let response = client
                    .get(url.clone())
                    .send()
                    .await
                    .expect("failed get request");

                match response.status() {
                    StatusCode::OK => {
                        match response.json::<Currency>().await {
                            Ok(res) => {
                                
                                let crypto = make(res.data);
                                
                                if let Err(_) = tx.send(crypto).await {
                                    println!("send error in crypto(VCManager)");
                                    break 'outer;
                                }
                            }
                            Err(e) => {
                                panic!("Couldn't convert json: {:?}", &e);
                            }
                        }
                    }
                    StatusCode::UNAUTHORIZED => {
                        panic!("Need to another token");
                    }
                    other => {
                        panic!("Uh oh! Something unexpected happened: {:?}", other)
                    }
                }

                sleep(Duration::from_secs(crypto_update)).await;
            }
        });

        VCManager {
            running_flag,
            task,
            crypto_store: BTreeMap::new(),
            rx,
            crypto_data_send_tx,
        }
    }

    pub async fn update_crypto_store(&mut self) -> Result<()> {

        loop {
            if !self.running_flag.load(Ordering::Relaxed) {
                break;
            }

            match self.rx.recv().await {
                Some(data) => {
                    self.update(data).await.expect("failed update cryptocurrency");

                    self.send_crypto_ranking().await.expect("failed send crypto data to other thread");
                }
                None => {
                    println!("connection refuse");
                    break;
                }
            }
        }

        Err(anyhow::anyhow!("connection refuse"))
    }

    async fn update(&mut self, data: Vec<CryptoCurrencyModel>) -> Result<()> {

        data.into_iter().for_each(|crypto| {
            if let Some(_v) = self.crypto_store.get_mut(&crypto.symbol) {

            }
        });

        Ok(())
    }

    pub async fn send_crypto_ranking(&self) -> Result<()> {
        if let Err(e) = self.crypto_data_send_tx.send(1).await {
            bail!("failed send: {:?}", &e);
        }

        Ok(())
    }
}
