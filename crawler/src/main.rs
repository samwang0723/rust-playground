mod engine;
use crate::engine::fetcher::{fetch_content, Payload};
use crate::engine::model::Model;
use crate::engine::parser::{ConcentrationStrategy, Parser};

use chrono::{Datelike, Local};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    tracing_subscriber::fmt::init();

    let proxy_api_key = env::var("PROXY_API_KEY").unwrap();
    // List of URLs to process
    let stocks = vec!["2330", "2363", "8150"];

    let capacity = stocks.len() * 5;
    let (url_tx, url_rx) = mpsc::channel(capacity);

    // retrieve all handles and ensure process not termiated before tasks completed
    let url_generation_handle = tokio::spawn(generate_urls(proxy_api_key, stocks.clone(), url_tx));
    let parse_aggregate_handle = tokio::spawn(parse_data(url_rx, capacity));

    // Await on both handles to ensure completion
    let _results = tokio::try_join!(url_generation_handle, parse_aggregate_handle);
}

async fn generate_urls(proxy_api_key: String, stocks: Vec<&str>, sender: mpsc::Sender<String>) {
    for stock in stocks.iter() {
        for i in 1..=6 {
            // skip the 40 days calculation
            if i == 5 {
                continue;
            }

            let url = format!(
                "https://api.webscrapingapi.com/v1?url=https://fubon-ebrokerdj.fbs.com.tw/z/zc/zco/zco_{}_{}.djhtm&api_key={}",
                stock, i, proxy_api_key
            );

            sender.send(url).await.unwrap();
        }
    }

    drop(sender);
}

async fn parse_data(mut url_rx: mpsc::Receiver<String>, capacity: usize) {
    let semaphore = Arc::new(Semaphore::new(50));
    let (content_tx, content_rx) = mpsc::channel(capacity);

    let fetch_handle = tokio::spawn(async move {
        while let Some(url) = url_rx.recv().await {
            print!(".");
            io::stdout().flush().unwrap();

            let sem_clone = Arc::clone(&semaphore);
            let content_tx_clone = content_tx.clone();
            tokio::spawn(async move {
                let _permit = sem_clone
                    .acquire()
                    .await
                    .expect("Failed to acquire semaphore permit");

                match fetch_content(url.clone()).await {
                    Ok(payload) => {
                        print!("_");
                        io::stdout().flush().unwrap();

                        if let Err(e) = content_tx_clone.send(payload).await {
                            eprintln!("Failed to send content: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch content for URL {}: {}", url, e);
                        // continue to the next URL without sending anything to content_tx
                    }
                }
                // Permit is automatically released when dropped
            });
        }
    });

    let aggregate_handle = tokio::spawn(aggregate(content_rx));

    // Await on both handles to ensure completion
    let _results = tokio::try_join!(fetch_handle, aggregate_handle);
}

async fn aggregate(mut content_rx: mpsc::Receiver<Payload>) {
    let today = Local::now();
    let formatted_date = format!("{}{:02}{:02}", today.year(), today.month(), today.day());
    let mut stock_map: HashMap<String, Model> = HashMap::new();

    while let Some(payload) = content_rx.recv().await {
        let url = payload.source.clone();
        let parser = Parser::new(ConcentrationStrategy);
        let res = parser.parse(payload).await;

        if let Ok(res_value) = res {
            let model = stock_map
                .entry(res_value.0.clone())
                .or_insert_with(|| Model {
                    stock_id: res_value.0,
                    exchange_date: formatted_date.clone(),
                    concentration: vec![0; 5],
                });
            model.concentration[res_value.1] = res_value.2;
        } else if let Err(e) = res {
            eprintln!("Failed to parse content for URL {}: {}", url, e);
        }
    }

    // extract items from map and print json string
    for (_, model) in stock_map.iter() {
        println!("{}", model.to_json().unwrap());
    }
}
