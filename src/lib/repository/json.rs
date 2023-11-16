use serde::Deserialize;

#[derive(Deserialize)]
pub struct JSONFinance {
    pub products: Vec<JSONProduct>,
    pub logs: Vec<JSONLog>,
}

#[derive(Deserialize)]
pub struct JSONProduct {
    pub product: String,
    pub category: String,
}

#[derive(Deserialize)]
pub struct JSONLog {
    pub product: String,
    pub price: f64,
}
