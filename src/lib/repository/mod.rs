use std::env;
use std::fs;
use std::io::Write;

use self::json::JSONFinance;
use crate::finance::Finance;

mod json;

pub trait FinanceRepository: Clone {
    fn load(&self) -> Result<Finance, String>;
    fn save(&self, finance: &Finance) -> Result<(), String>;
}

#[derive(Clone, Debug)]
pub struct EnvJSONFinanceRepository {
    json_path: String,
}

impl EnvJSONFinanceRepository {
    pub fn from_env() -> Result<Self, String> {
        let json_path = env::var("FINANCE_FILE_PATH")
            .map_err(|_| "Couldn't load FINANCE_FILE_PATH variable! Is is set?")?;

        Ok(Self { json_path })
    }
}

impl FinanceRepository for EnvJSONFinanceRepository {
    fn load(&self) -> Result<Finance, String> {
        let json_path = &self.json_path;
        let json_content = fs::read_to_string(json_path)
            .map_err(|_| format!("Couldn't read Finance file {json_path}. Does it exist?"))?;

        let json_finance: JSONFinance = serde_json::from_str(&json_content).map_err(|_| {
            "Error parsing Finance from JSON content. Does it have the correct structure?"
        })?;

        Ok(json_finance.to_finance())
    }

    fn save(&self, finance: &Finance) -> Result<(), String> {
        let json_path = &self.json_path;
        let file = fs::File::create(json_path).map_err(|_| {
            format!("Couldn't write to file {json_path}! Does the directory exist?")
        })?;

        let json_finance = JSONFinance::from_finance(finance);
        writeln!(
            &file,
            "{}",
            serde_json::to_string(&json_finance).map_err(|_| "Unknown error.")?
        )
        .map_err(|_| "Unknown error.")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use ::json::parse as json_parse;
    use tempfile::TempDir;

    use super::*;
    use crate::finance::FinanceLog;

    #[test]
    fn test_from_env_loads_correctly() {
        with_valid_temp_finance_json_file(|json_file_path| {
            set_finance_file_path(json_file_path.to_str().unwrap());

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
        with_invalid_temp_finance_json_file(|json_file_path| {
            set_finance_file_path(json_file_path.to_str().unwrap());

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
        set_finance_file_path("inexistent-file.json");

        let repo = EnvJSONFinanceRepository::from_env().expect("Didn't expect from_env to fail!");
        let load_err = repo.load().expect_err("Expected load to fail!");

        assert_eq!(
            load_err,
            "Couldn't read Finance file inexistent-file.json. Does it exist?"
        );
    }

    #[test]
    fn test_from_env_save() {
        let dir = TempDir::new().unwrap();
        let finance_path = dir.path().join("finance.json");
        set_finance_file_path(finance_path.to_str().unwrap());

        let repo = EnvJSONFinanceRepository::from_env().expect("Didn't expect from_env to fail!");
        let finance = Finance::new()
            .with_product("prod1", "cat1")
            .with_log("prod1", 10.0);

        repo.save(&finance).expect("Didn't expect save to fail!");

        let file_contents = fs::read_to_string(finance_path).expect("Error reading file!");

        let expected_json = json_parse(
            r#"{ "products": [{ "product": "prod1", "category": "cat1" }], "logs": [{"product": "prod1", "price": 10}] }"#,
        ).unwrap();

        assert_eq!(
            json_parse(&file_contents).expect("Error loading saved JSON Finance file!"),
            expected_json
        );
    }

    #[test]
    fn test_from_env_save_file_err() {
        set_finance_file_path("/inexistent-dir/file.json");

        let repo = EnvJSONFinanceRepository::from_env().expect("Didn't expect from_env to fail!");
        let finance = Finance::new()
            .with_product("prod1", "cat1")
            .with_log("prod1", 10.0);

        let save_err = repo.save(&finance).expect_err("Expected save to fail!");
        assert_eq!(
            save_err,
            "Couldn't write to file /inexistent-dir/file.json! Does the directory exist?"
        );
    }

    fn set_finance_file_path(value: &str) {
        env::set_var("FINANCE_FILE_PATH", value);
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

        assert_eq!(
            loaded_finance.logs[0],
            FinanceLog {
                product: "prod1".to_owned(),
                price: 10.0
            }
        );
        assert_eq!(
            loaded_finance.logs[1],
            FinanceLog {
                product: "prod2".to_owned(),
                price: 20.0
            }
        );
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

    fn with_valid_temp_finance_json_file<F>(func: F)
    where
        F: Fn(std::path::PathBuf),
    {
        with_temp_finance_json_file(&json_finance_content(), func);
    }

    fn with_invalid_temp_finance_json_file<F>(func: F)
    where
        F: Fn(std::path::PathBuf),
    {
        with_temp_finance_json_file("invalid finance json", func);
    }

    fn with_temp_finance_json_file<F>(finance_json: &str, func: F)
    where
        F: Fn(std::path::PathBuf),
    {
        let dir = TempDir::new().expect("Error creating temp dir!");
        let file_path = dir.path().join("data_file.json");
        let mut file = File::create(&file_path).expect("Error creating temp JSON file!");

        writeln!(file, "{}", finance_json).expect("Error writing to temp JSON file!");

        func(file_path)
    }
}
