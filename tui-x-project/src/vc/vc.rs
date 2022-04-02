//
use super::CryptoCurrencyModel;
use anyhow::Result;
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
    sync::mpsc::{self, Receiver},
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
    pub task: tokio::task::JoinHandle<()>,
    crypto_store: BTreeMap<String, Vec<CryptoCurrencyModel>>,
    rx: Receiver<u64>,
}

impl VCManager {
    pub fn new(
        running_flag: Arc<AtomicBool>,
        crypto_update: u64,
        api_key: &'static str,
        url: String,
    ) -> VCManager {
        let (tx, rx) = mpsc::channel(1);

        let task = tokio::spawn(async move {
            let clone_flag = Arc::clone(&running_flag);

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
                                res.data.iter().for_each(|i| {
                                    println!("Success! {:?}", i);
                                })
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

                if let Err(_) = tx.send(1).await {
                    println!("send error in crypto(VCManager)");
                    break 'outer;
                }

                sleep(Duration::from_secs(crypto_update)).await;
            }
        });

        VCManager {
            task,
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
