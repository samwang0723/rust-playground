mod engine;
use crate::engine::fetcher::fetch_content;
use crate::engine::parser::{ConcentrationStrategy, Parser};

use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // List of URLs to process
    let urls: Vec<String> = vec![
        "https://fubon-ebrokerdj.fbs.com.tw/z/zc/zco/zco_3704_1.djhtm".to_string(),
        "https://fubon-ebrokerdj.fbs.com.tw/z/zc/zco/zco_3704_2.djhtm".to_string(),
        "https://fubon-ebrokerdj.fbs.com.tw/z/zc/zco/zco_3704_3.djhtm".to_string(),
        "https://fubon-ebrokerdj.fbs.com.tw/z/zc/zco/zco_3704_4.djhtm".to_string(),
        "https://fubon-ebrokerdj.fbs.com.tw/z/zc/zco/zco_3704_5.djhtm".to_string(),
        // ...add more URLs here
    ];

    // Call the task_management function to process the URLs
    task_management(urls).await;
}

async fn task_management(urls: Vec<String>) {
    // Create a semaphore with 3 permits
    let semaphore = Arc::new(Semaphore::new(3));

    // Create a vector to hold the task handles
    let mut handles = Vec::new();

    for url in urls {
        // Clone the Arc containing the semaphore for the task
        let sem_clone = Arc::clone(&semaphore);

        // Spawn a new task
        let handle = task::spawn(async move {
            // Acquire a permit from the semaphore
            let _permit = sem_clone.acquire().await.unwrap();
            // Now this task holds a permit, so only 3 tasks can hold a permit at a time

            let body = fetch_content(url.to_owned()).await.unwrap();
            let parser = Parser::new(ConcentrationStrategy);
            match parser.parse(&body.content).await {
                Ok(res) => println!("res: {}", res),
                Err(_e) => (),
            };

            // For example, you can print the URL being processed:
            println!("Processed URL: {}", url);
        });

        // Store the handle so we can await it later
        handles.push(handle);
    }

    // Await all the tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    println!("All tasks complete");
}
