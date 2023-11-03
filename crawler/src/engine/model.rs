use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
pub struct Model {
    pub stock_id: String,
    pub exchange_date: String,
    pub concentration: Vec<i32>,
}

impl Model {
    // Define a method to convert the struct into a JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Debug)]
pub struct ProcCon(pub String, pub usize, pub i32);
