use gregorian::{Month, Year};
use serde::{Deserialize, Serialize};

use crate::finance::{Finance, FinanceLog};

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
                    product: log.product.to_owned(),
                    price: log.price,
                    year: log.year_month.year().to_number(),
                    month: log.year_month.month().to_number(),
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
            finance = finance.with_log(FinanceLog {
                product: json_log.product.to_owned(),
                price: json_log.price,
                year_month: gregorian::YearMonth::new(
                    Year::new(json_log.year),
                    Month::new(json_log.month).unwrap(), // TODO Test unwrap
                ),
            })
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
    pub year: i16,
    pub month: u8,
}
