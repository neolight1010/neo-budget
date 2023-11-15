use std::env;
use std::fs;
use std::fs::File;

use serde::Deserialize;

use crate::Finance;

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

struct JSONFinanceLoader {
    json: String,
}

impl JSONFinanceLoader {
    fn new(json: &str) -> Self {
        Self {
            json: json.to_string(),
        }
    }

    fn from_env() -> Result<Self, String> {
        let json_file_path = env::var("FINANCE_FILE_PATH").map_err(|_| "")?;
        let json_content = fs::read_to_string(json_file_path).map_err(|_| "")?;

        Ok(Self { json: json_content })
    }

    fn load(&self) -> Result<Finance, serde_json::Error> {
        let json_finance: JSONFinance = serde_json::from_str(&self.json)?;

        let mut finance = Finance::new();
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
    use std::fs::File;
    use std::io::Write;

    use tempfile::TempDir;

    use super::*;

    fn json_finance_content() -> String {
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
"#
        .to_string()
    }

    fn assert_finance_is_loaded_correctly(loaded_finance: &Finance) {
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
    fn test_loader_ok() {
        let loader = JSONFinanceLoader::new(&json_finance_content());
        let loaded_finance = loader.load().unwrap();

        assert_finance_is_loaded_correctly(&loaded_finance);
    }

    #[test]
    fn test_loader_err() {
        let loader = JSONFinanceLoader::new("invalid json");
        assert!(loader.load().is_err());
    }

    #[test]
    fn test_from_env() -> Result<(), String> {
        let dir = TempDir::new().map_err(|_| "Error creating temp dir!")?;
        let json_file_path = write_finance_json_in(&dir)?;

        env::set_var("FINANCE_FILE_PATH", &json_file_path);

        let loader = JSONFinanceLoader::from_env()
            .map_err(|_| "Error creating JsonFinanceLoader from env!")?;

        let loaded_finance = loader
            .load()
            .map_err(|_| "Error loading Finance from env!")?;

        assert_finance_is_loaded_correctly(&loaded_finance);

        Ok(())
        // TODO Test VarError
        // TODO Test open error
        // TODO Test read error
    }

    fn write_finance_json_in(dir: &TempDir) -> Result<std::path::PathBuf, String> {
        let file_path = dir.path().join("data_file.json");

        let mut file = File::create(&file_path).map_err(|_| "Error creating temp JSON file!")?;

        writeln!(file, "{}", json_finance_content())
            .map_err(|_| "Error writing to temp JSON file!")?;

        Ok(file_path)
    }
}
