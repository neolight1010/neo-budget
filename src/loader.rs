use serde::Deserialize;

use crate::ExpenditureLog;

#[derive(Deserialize)]
struct JSONProduct {
    product: String,
    category: String,
}

#[derive(Deserialize)]
struct JSONLog {
    product: String,
    price: f64,
}

#[derive(Deserialize)]
struct JSONFinance {
    products: Vec<JSONProduct>,
    logs: Vec<JSONLog>,
}

struct JsonFinanceLoader {
    json: String,
}

impl JsonFinanceLoader {
    fn new(json: &str) -> Self {
        Self {
            json: json.to_string(),
        }
    }

    fn load(&self) -> Result<ExpenditureLog, serde_json::Error> {
        let json_finance: JSONFinance = serde_json::from_str(&self.json)?;

        let mut finance = ExpenditureLog::new();
        for json_product in json_finance.products.iter() {
            finance = finance.with_product(&json_product.product, &json_product.category);
        }

        for json_log in json_finance.logs.iter() {
            finance = finance.with_log(&json_log.product, json_log.price);
        }

        Ok(finance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_ok() {
        let loader = JsonFinanceLoader::new(
            r#"
{
    "products": [
        {
            "product": "prod1",
            "category": "cat1"
        },
        {
            "product": "prod2",
            "category": "cat2"
        }
    ],
    "logs": [
        {
            "product": "prod1",
            "price": 10
        },
        {
            "product": "prod2",
            "price": 20
        }
    ]
}
"#,
        );

        let loaded_finance = loader.load().unwrap();

        assert_eq!(
            loaded_finance.product_categories.get("prod1"),
            Some(&"cat1".to_string())
        );

        assert_eq!(
            loaded_finance.product_categories.get("prod2"),
            Some(&"cat2".to_string())
        );

        assert_eq!(loaded_finance.logs[0], ("prod1".to_owned(), 10.0));
        assert_eq!(loaded_finance.logs[1], ("prod2".to_owned(), 20.0));
    }

    #[test]
    fn test_loader_err() {
        let loader = JsonFinanceLoader::new("invalid json");
        assert!(loader.load().is_err());
    }
}
