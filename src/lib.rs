use std::collections::HashMap;

struct ExpenditureLog {
    product_to_total: HashMap<String, f64>,
    category_to_total: HashMap<String, f64>,

    product_to_category: HashMap<String, String>,
}

impl ExpenditureLog {
    pub fn new() -> Self {
        Self {
            product_to_total: HashMap::new(),
            category_to_total: HashMap::new(),

            product_to_category: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: &str, category: &str) {
        self.product_to_category
            .insert(product.to_owned(), category.to_owned());
    }

    pub fn add_log(&mut self, product: &str, price: f64) {
        increase_total_or_insert(&mut self.product_to_total, product, price);

        if let Some(category) = self.product_to_category.get(product) {
            increase_total_or_insert(&mut self.category_to_total, category, price);
        }
    }

    pub fn product_total(&self, product: &str) -> f64 {
        self.product_to_total
            .get(product)
            .unwrap_or(&0.0)
            .to_owned()
    }

    pub fn category_total(&self, category: &str) -> f64 {
        self.category_to_total
            .get(category)
            .unwrap_or(&0.0)
            .to_owned()
    }
}

fn increase_total_or_insert(total_map: &mut HashMap<String, f64>, key: &str, addend: f64) {
    let current_total = total_map.get(key).unwrap_or(&0.0);

    total_map.insert(key.to_owned(), current_total + addend);
}

#[cfg(test)]
mod tests {
    use super::ExpenditureLog;

    #[test]
    fn expenditure_log_product_total() {
        let mut expenditure_log = ExpenditureLog::new();

        expenditure_log.add_log("prod1", 10.0);

        assert_eq!(expenditure_log.product_total("prod1"), 10.0);
    }

    #[test]
    fn expenditure_log_category_total() {
        let mut expenditure_log = ExpenditureLog::new();

        expenditure_log.add_product("prod1", "cat1");
        expenditure_log.add_log("prod1", 10.0);

        assert_eq!(expenditure_log.category_total("cat1"), 10.0);
    }
}
