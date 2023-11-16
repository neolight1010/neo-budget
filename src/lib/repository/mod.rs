use std::env;
use std::fs;

use self::json::JSONFinance;
use crate::Finance;

mod json;

#[derive(Debug)]
pub struct JSONFinanceRepository {
    json: String,
}

impl JSONFinanceRepository {
    fn new(json: &str) -> Self {
        Self {
            json: json.to_string(),
        }
    }

    pub fn from_env() -> Result<Self, String> {
        let json_file_path = env::var("FINANCE_FILE_PATH")
            .map_err(|_| "Couldn't load FINANCE_FILE_PATH variable! Is is set?")?;
        let json_content = fs::read_to_string(&json_file_path)
            .map_err(|_| format!("Couldn't read Finance file {json_file_path}. Does it exist?"))?;

        Ok(Self::new(&json_content))
    }

    pub fn load(&self) -> Result<Finance, String> {
        let json_finance: JSONFinance = serde_json::from_str(&self.json).map_err(|_| {
            "Error parsing Finance from JSON content. Does it have the correct structure?"
        })?;

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
        let loader = JSONFinanceRepository::new(&json_finance_content());
        let loaded_finance = loader.load().unwrap();

        assert_finance_is_loaded_correctly(&loaded_finance);
    }

    #[test]
    fn test_loader_err() {
        let loader = JSONFinanceRepository::new("invalid json");
        assert_eq!(
            loader.load().expect_err("Expected load to fail!"),
            "Error parsing Finance from JSON content. Does it have the correct structure?"
        );
    }

    #[test]
    fn test_from_env() -> Result<(), String> {
        let dir = TempDir::new().map_err(|_| "Error creating temp dir!")?;
        let json_file_path = write_finance_json_in(&dir)?;

        env::set_var("FINANCE_FILE_PATH", &json_file_path);

        let loader = JSONFinanceRepository::from_env()
            .map_err(|_| "Error creating JsonFinanceLoader from env!")?;

        let loaded_finance = loader
            .load()
            .map_err(|_| "Error loading Finance from env!")?;

        assert_finance_is_loaded_correctly(&loaded_finance);

        Ok(())
        // TODO Test read error
    }

    #[test]
    fn test_from_env_env_var_err() {
        env::remove_var("FINANCE_FILE_PATH");
        let loader_err = JSONFinanceRepository::from_env().expect_err("Expected from_env to fail!");

        assert_eq!(
            loader_err,
            "Couldn't load FINANCE_FILE_PATH variable! Is is set?".to_string()
        );
    }

    #[test]
    fn test_from_env_file_open_err() {
        env::set_var("FINANCE_FILE_PATH", "./inexistent-file.json");

        let loader_err = JSONFinanceRepository::from_env().expect_err("Expected from_env to fail!");

        assert_eq!(
            loader_err,
            "Couldn't read Finance file ./inexistent-file.json. Does it exist?"
        );
    }

    fn write_finance_json_in(dir: &TempDir) -> Result<std::path::PathBuf, String> {
        let file_path = dir.path().join("data_file.json");

        let mut file = File::create(&file_path).map_err(|_| "Error creating temp JSON file!")?;

        writeln!(file, "{}", json_finance_content())
            .map_err(|_| "Error writing to temp JSON file!")?;

        Ok(file_path)
    }
}
