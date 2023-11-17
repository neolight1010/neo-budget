use std::env;
use std::fs;

use self::json::JSONFinance;
use crate::Finance;

mod json;

#[derive(Debug)]
pub struct EnvJSONFinanceRepository {
    json_path: String,
}

impl EnvJSONFinanceRepository {
    pub fn from_env() -> Result<Self, String> {
        let json_path = env::var("FINANCE_FILE_PATH")
            .map_err(|_| "Couldn't load FINANCE_FILE_PATH variable! Is is set?")?;

        Ok(Self { json_path })
    }

    pub fn load(&self) -> Result<Finance, String> {
        let json_path = &self.json_path;
        let json_content = fs::read_to_string(json_path)
            .map_err(|_| format!("Couldn't read Finance file {json_path}. Does it exist?"))?;

        let json_finance: JSONFinance = serde_json::from_str(&json_content).map_err(|_| {
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

    #[test]
    fn test_from_env_loads_correctly() {
        with_valid_finance_json_in_temp_dir(|json_file_path| {
            env::set_var("FINANCE_FILE_PATH", &json_file_path);

            let loader =
                EnvJSONFinanceRepository::from_env().expect("Didn't expect from_env to fail!");

            let loaded_finance = loader.load().expect("Didn't expect load to fail!");

            assert_finance_is_loaded_correctly(&loaded_finance);
        });
    }

    #[test]
    fn test_from_env_env_var_err() {
        env::remove_var("FINANCE_FILE_PATH");
        let loader_err =
            EnvJSONFinanceRepository::from_env().expect_err("Expected from_env to fail!");

        assert_eq!(
            loader_err,
            "Couldn't load FINANCE_FILE_PATH variable! Is is set?".to_string()
        );
    }

    #[test]
    fn test_loader_err_invalid_json() {
        with_invalid_finance_json_in_temp_dir(|json_file_path| {
            env::set_var("FINANCE_FILE_PATH", &json_file_path);

            let repo =
                EnvJSONFinanceRepository::from_env().expect("Didn't expect from_env to fail!");
            let load_err = repo.load().expect_err("Expected load to fail!");

            assert_eq!(
                load_err,
                "Error parsing Finance from JSON content. Does it have the correct structure?"
            );
        })
    }

    #[test]
    fn test_from_env_load_file_open_err() {
        env::set_var("FINANCE_FILE_PATH", "./inexistent-file.json");

        let repo = EnvJSONFinanceRepository::from_env().expect("Didn't expect from_env to fail!");
        let load_err = repo.load().expect_err("Expected load to fail!");

        assert_eq!(
            load_err,
            "Couldn't read Finance file ./inexistent-file.json. Does it exist?"
        );
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

    fn with_valid_finance_json_in_temp_dir<F>(func: F)
    where
        F: Fn(std::path::PathBuf) -> (),
    {
        with_finance_json_in_temp_dir(&json_finance_content(), func);
    }

    fn with_invalid_finance_json_in_temp_dir<F>(func: F)
    where
        F: Fn(std::path::PathBuf) -> (),
    {
        with_finance_json_in_temp_dir("invalid finance json", func);
    }

    fn with_finance_json_in_temp_dir<F>(finance_json: &str, func: F)
    where
        F: Fn(std::path::PathBuf) -> (),
    {
        let dir = TempDir::new().expect("Error creating temp dir!");
        let file_path = dir.path().join("data_file.json");
        let mut file = File::create(&file_path).expect("Error creating temp JSON file!");

        writeln!(file, "{}", finance_json).expect("Error writing to temp JSON file!");

        func(file_path)
    }
}
