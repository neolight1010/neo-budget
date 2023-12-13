use serde::{Deserialize, Serialize};

use crate::finance::Finance;

#[derive(Deserialize, Serialize)]
pub struct JSONFinance {
    pub products: Vec<JSONProduct>,
    pub logs: Vec<JSONLog>,
}

impl JSONFinance {
    pub fn from_finance(finance: &Finance) -> Self {
        Self {
            logs: finance
                .logs
                .iter()
                .map(|log| JSONLog {
                    product: log.0.clone(),
                    price: log.1,
                })
                .collect(),
            products: finance
                .product_categories
                .iter()
                .map(|(product, category)| JSONProduct {
                    product: product.clone(),
                    category: category.clone(),
                })
                .collect(),
        }
    }

    pub fn to_finance(&self) -> Finance {
        let mut finance = Finance::new();
        for json_product in self.products.iter() {
            finance = finance.with_product(&json_product.product, &json_product.category);
        }

        for json_log in self.logs.iter() {
            finance = finance.with_log(&json_log.product, json_log.price);
        }

        finance
    }
}

#[derive(Deserialize, Serialize)]
pub struct JSONProduct {
    pub product: String,
    pub category: String,
}

#[derive(Deserialize, Serialize)]
pub struct JSONLog {
    pub product: String,
    pub price: f64,
}
