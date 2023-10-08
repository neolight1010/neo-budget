use std::collections::HashMap;

pub type Price = f64;

pub struct ExpenditureLog {
    logs: Vec<(String, Price)>,
}

impl ExpenditureLog {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn with_log(self, product: &str, price: Price) -> Self {
        Self {
            logs: self
                .logs
                .into_iter()
                .chain(vec![(product.to_owned(), price)])
                .collect(),
            ..self
        }
    }
}

pub struct ExpenditureLogStats {
    log: ExpenditureLog,

    product_totals: HashMap<String, Price>,
    category_totals: HashMap<String, Price>,

    product_categories: HashMap<String, String>,
}

impl ExpenditureLogStats {
    pub fn new(log: ExpenditureLog) -> Self {
        Self {
            log,

            product_totals: HashMap::new(),
            category_totals: HashMap::new(),

            product_categories: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: &str, category: &str) {
        self.product_categories
            .insert(product.to_owned(), category.to_owned());
    }

    pub fn add_log(&mut self, product: &str, price: Price) {
        increase_total_or_insert(&mut self.product_totals, product, price);

        if let Some(category) = self.product_categories.get(product) {
            increase_total_or_insert(&mut self.category_totals, category, price);
        }
    }

    pub fn product_total(&self, product: &str) -> Price {
        self.log
            .logs
            .iter()
            .filter(|(p, _)| p == product)
            .map(|(_, p)| p)
            .sum()
    }

    pub fn product_totals(&self) -> HashMap<String, Price> {
        self.product_totals.clone()
    }

    pub fn category_total(&self, category: &str) -> Price {
        self.category_totals
            .get(category)
            .unwrap_or(&0.0)
            .to_owned()
    }

    pub fn category_totals(&self) -> HashMap<String, Price> {
        self.category_totals.clone()
    }
}

fn increase_total_or_insert(total_map: &mut HashMap<String, Price>, key: &str, addend: Price) {
    let current_total = total_map.get(key).unwrap_or(&0.0);

    total_map.insert(key.to_owned(), current_total + addend);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{ExpenditureLog, ExpenditureLogStats};

    #[test]
    fn expenditure_log_product_total() {
        let log = ExpenditureLog::new().with_log("prod1", 10.0);
        let stats = ExpenditureLogStats::new(log);

        assert_eq!(stats.product_total("prod1"), 10.0);
    }

    #[test]
    fn expenditure_log_category_total() {
        let mut expenditure_log = ExpenditureLogStats::new(ExpenditureLog::new());

        expenditure_log.add_product("prod1", "cat1");
        expenditure_log.add_log("prod1", 10.0);

        assert_eq!(expenditure_log.category_total("cat1"), 10.0);
    }

    #[test]
    fn product_totals() {
        let mut expenditure_log = ExpenditureLogStats::new(ExpenditureLog::new());

        expenditure_log.add_log("prod1", 10.0);

        let mut expected_totals = HashMap::new();
        expected_totals.insert("prod1".to_owned(), 10.0);

        assert_eq!(expenditure_log.product_totals(), expected_totals);
    }

    #[test]
    fn category_totals() {
        let mut expenditure_log = ExpenditureLogStats::new(ExpenditureLog::new());

        expenditure_log.add_product("prod1", "cat1");
        expenditure_log.add_log("prod1", 10.0);

        let mut expected_totals = HashMap::new();
        expected_totals.insert("cat1".to_owned(), 10.0);

        assert_eq!(expenditure_log.category_totals(), expected_totals);
    }
}
