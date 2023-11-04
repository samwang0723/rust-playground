use anyhow::{anyhow, Result};
use async_trait::async_trait;
use regex::Regex;
use scraper::{Html, Selector};

use super::*;

#[async_trait]
pub trait ParseStrategy: Conversion {
    type Error;
    type Input;
    // Declare an associated type that will be the return type of the parse method.
    type Output;

    async fn parse(&self, payload: Self::Input) -> Result<Self::Output, Self::Error>;
}

pub trait Conversion {
    fn to_i32(&self, data: &str) -> Result<i32, anyhow::Error>;
    fn to_usize(&self, data: &str) -> Result<usize, anyhow::Error>;
}

#[derive(Debug)]
pub struct DailyCloseStrategy;

#[async_trait]
impl ParseStrategy for DailyCloseStrategy {
    type Error = anyhow::Error;
    type Input = String;
    type Output = String;

    async fn parse(&self, _payload: Self::Input) -> Result<Self::Output, Self::Error> {
        Err(anyhow!("DailyCloseStrategy parse not yet implemented"))
    }
}

impl Conversion for DailyCloseStrategy {
    fn to_i32(&self, _data: &str) -> Result<i32, anyhow::Error> {
        Err(anyhow!("DailyCloseStrategy to_i32 not yet implemented"))
    }

    fn to_usize(&self, _data: &str) -> Result<usize, anyhow::Error> {
        Err(anyhow!("DailyCloseStrategy to_usize not yet implemented"))
    }
}

#[derive(Debug)]
pub struct ConcentrationStrategy;

impl ConcentrationStrategy {
    fn identifier(&self, url: String) -> Result<(String, usize), anyhow::Error> {
        let re = Regex::new(r"zco_(\d+)_(\d+)")?;
        let captures = match re.captures(&url) {
            Some(captures) => captures,
            None => {
                return Err(anyhow!("Invalid URL"));
            }
        };

        let stock_id = captures.get(1).map_or("", |m| m.as_str());
        let index = self.to_usize(captures.get(2).map_or("", |m| m.as_str()))?;

        // backfill the missing index 4 (40 days replaced with 60 days)
        let pos = if index == 6 { index - 2 } else { index - 1 };

        Ok((stock_id.to_string(), pos))
    }
}

#[async_trait]
impl ParseStrategy for ConcentrationStrategy {
    type Error = anyhow::Error;
    type Input = fetcher::Payload;
    type Output = model::Concentration;

    async fn parse(&self, payload: Self::Input) -> Result<Self::Output, Self::Error> {
        let (stock_id, pos) = match self.identifier(payload.source.clone()) {
            Ok((stock_id, pos)) => (stock_id, pos),
            Err(e) => {
                return Err(anyhow!("Failed to parse URL: {}", e));
            }
        };

        let fragment = Html::parse_document(payload.content.as_str());
        let td_selector = match Selector::parse("td") {
            Ok(selector) => selector,
            Err(e) => {
                return Err(anyhow!("Failed to create selector: {}", e));
            }
        };

        let mut elements = fragment.select(&td_selector);
        let mut total_buy = 0;
        let mut total_sell = 0;
        while let Some(element) = elements.next() {
            let text = element.text().collect::<Vec<_>>().join("");

            // unfortunately, proxy service encoded big5 into utf-8 and cannot be reverted
            // directly match the utf-8 string is the fastest way
            if text == "合計買超張數" || text == "�X�p�R�W�i��" {
                if let Some(next_element) = elements.next() {
                    let buy = next_element.text().collect::<Vec<_>>().join("");
                    total_buy = self.to_i32(&buy)?;
                }
            } else if text == "合計賣超張數" || text == "�X�p��W�i��" {
                if let Some(next_element) = elements.next() {
                    let sell = next_element.text().collect::<Vec<_>>().join("");
                    total_sell = self.to_i32(&sell)?;
                }
            }
        }

        Ok(model::Concentration(stock_id, pos, total_buy - total_sell))
    }
}

impl Conversion for ConcentrationStrategy {
    fn to_i32(&self, data: &str) -> Result<i32, anyhow::Error> {
        let without_comma = data.replace(',', ""); // This will do nothing if there is no comma
        without_comma.parse::<i32>().map_err(|e| anyhow!(e))
    }

    fn to_usize(&self, data: &str) -> Result<usize, anyhow::Error> {
        data.parse::<usize>().map_err(|e| anyhow!(e))
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

    pub async fn parse(&self, payload: T::Input) -> Result<T::Output, T::Error> {
        self.strategy.parse(payload).await
    }
}
