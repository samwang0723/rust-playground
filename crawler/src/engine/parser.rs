use anyhow::{anyhow, Result};
use async_trait::async_trait;
use scraper::{Html, Selector};

#[async_trait]
pub trait ParseStrategy {
    type Error;
    async fn parse(&self, payload: &str) -> Result<String, Self::Error>;
}

#[derive(Debug)]
pub struct DailyCloseStrategy;

#[async_trait]
impl ParseStrategy for DailyCloseStrategy {
    type Error = anyhow::Error;

    async fn parse(&self, _payload: &str) -> Result<String, Self::Error> {
        Err(anyhow!("DailyCloseStrategy not yet implemented"))
    }
}

#[derive(Debug)]
pub struct ConcentrationStrategy;

#[async_trait]
impl ParseStrategy for ConcentrationStrategy {
    type Error = anyhow::Error;

    async fn parse(&self, payload: &str) -> Result<String, Self::Error> {
        let fragment = Html::parse_document(payload);
        let td_selector = match Selector::parse("td") {
            Ok(selector) => selector,
            Err(e) => {
                return Err(anyhow!("Failed to create selector: {}", e));
            }
        };

        let mut elements = fragment.select(&td_selector);
        let mut total_buy = String::new();
        let mut total_sell = String::new();
        while let Some(element) = elements.next() {
            let text = element.text().collect::<Vec<_>>().join("");

            // unfortunately, proxy service encoded big5 into utf-8 and cannot be reverted
            // directly match the utf-8 string is the fastest way
            if text == "合計買超張數" || text == "�X�p�R�W�i��" {
                if let Some(next_element) = elements.next() {
                    total_buy = next_element.text().collect::<Vec<_>>().join("");
                }
            } else if text == "合計賣超張數" || text == "�X�p��W�i��" {
                if let Some(next_element) = elements.next() {
                    total_sell = next_element.text().collect::<Vec<_>>().join("");
                }
            }
        }
        Ok(format!(
            "total_buy: {}, total_sell: {}",
            total_buy, total_sell
        ))
    }
}

#[derive(Debug)]
pub struct Parser<T: ParseStrategy> {
    strategy: T,
}

impl<T: ParseStrategy> Parser<T> {
    pub fn new(strategy: T) -> Self {
        Self { strategy }
    }

    pub async fn parse(&self, payload: &str) -> Result<String, T::Error> {
        self.strategy.parse(payload).await
    }
}