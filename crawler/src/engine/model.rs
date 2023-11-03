#[derive(Debug)]
pub struct Model {
    pub stock_id: String,
    pub exchange_date: String,
    pub concentration: Vec<i32>,
}

#[derive(Debug)]
pub struct ProcCon(pub String, pub i32, pub i32);
