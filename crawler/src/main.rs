use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
struct ParsedData {
    total_buy: String,
    total_sell: String,
}

async fn fetch_url(url: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
}

async fn parse_html(html_str: &str) -> Result<ParsedData, Box<dyn Error>> {
    let fragment = Html::parse_document(html_str);
    let td_selector = match Selector::parse("td") {
        Ok(selector) => selector,
        Err(e) => {
            eprintln!("Failed to create selector: {}", e);
            return Err(Box::new(e));
        }
    };

    let mut elements = fragment.select(&td_selector);
    let mut total_buy = String::new();
    let mut total_sell = String::new();

    while let Some(element) = elements.next() {
        let text = element.text().collect::<Vec<_>>().join("");
        if text == "合計買超張數" {
            if let Some(next_element) = elements.next() {
                total_buy = next_element.text().collect::<Vec<_>>().join("");
            }
        } else if text == "合計賣超張數" {
            if let Some(next_element) = elements.next() {
                total_sell = next_element.text().collect::<Vec<_>>().join("");
            }
        }
    }

    Ok(ParsedData {
        total_buy,
        total_sell,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let url = "https://fubon-ebrokerdj.fbs.com.tw/z/zc/zco/zco_3704_1.djhtm";
    let body = match fetch_url(url).await {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Failed to fetch url: {}", e);
            return Err(e);
        }
    };

    let parsed_data = parse_html(&body).await?;
    println!(
        "total_buy: {}, total_sell: {}",
        parsed_data.total_buy, parsed_data.total_sell
    );

    Ok(())
}
