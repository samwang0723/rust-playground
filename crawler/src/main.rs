mod engine;
use crate::engine::fetcher::fetch_content;
use crate::engine::model::Model;
use crate::engine::parser::{ConcentrationStrategy, Parser};

use chrono::{Datelike, Local};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::task;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    tracing_subscriber::fmt::init();

    let proxy_api_key = env::var("PROXY_API_KEY").unwrap();
    // List of URLs to process
    let stocks = ["2330", "2363", "8150"];
    let mut all_urls = Vec::new();
    for stock in stocks.iter() {
        let urls = generate_urls(proxy_api_key.as_str(), stock);
        all_urls.extend(urls);
    }

    // Call the task_management function to process the URLs
    task_management(all_urls).await;
}

fn generate_urls(proxy_api_key: &str, stock_id: &str) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    for i in 1..=5 {
        urls.push(format!(
            "https://api.webscrapingapi.com/v1?url=https://fubon-ebrokerdj.fbs.com.tw/z/zc/zco/zco_{}_{}.djhtm&api_key={}",
            stock_id, i, proxy_api_key
        ));
    }
    urls
}

async fn task_management(urls: Vec<String>) {
    // Create a semaphore with 3 permits
    let semaphore = Arc::new(Semaphore::new(50));

    // Channel for collecting results
    let (tx, mut rx) = mpsc::channel(urls.len());

    for url in urls {
        // Clone the Arc containing the semaphore for the task
        let sem_clone = Arc::clone(&semaphore);
        let tx_clone = tx.clone();

        // Spawn a new task
        task::spawn(async move {
            // Acquire a permit from the semaphore
            let permit = sem_clone.acquire().await.unwrap();
            // Now this task holds a permit, so only 3 tasks can hold a permit at a time

            let body = fetch_content(url.to_owned()).await.unwrap();
            let parser = Parser::new(ConcentrationStrategy);
            let res = parser.parse(body).await.unwrap();
            drop(permit);

            tx_clone.send(res).await.unwrap();

            // For example, you can print the URL being processed:
            println!("Processed URL: {}", url);
        });
    }

    // Collect results
    drop(tx); // Close the sender to let the receiver know when all messages are sent

    let today = Local::now();
    let formatted_date = format!("{}{:02}{:02}", today.year(), today.month(), today.day());
    let mut stock_map: HashMap<String, Model> = HashMap::new();

    while let Some(proc_con) = rx.recv().await {
        let model = stock_map
            .entry(proc_con.0.clone())
            .or_insert_with(|| Model {
                stock_id: proc_con.0,
                exchange_date: formatted_date.clone(),
                concentration: vec![0; 5],
            });
        let i: usize = usize::from((proc_con.1 - 1) as u8);
        model.concentration[i] = proc_con.2;
    }

    println!("All tasks complete: {:?}", stock_map)
}
