use std::collections::HashMap;

fn main() {}

struct ExpenditureLog {
    product_to_total: HashMap<String, f64>,
}

impl ExpenditureLog {
    pub fn new() -> Self {
        Self {
            product_to_total: HashMap::new(),
        }
    }

    pub fn add_log(&mut self, product: &str, price: f64) {
        let current_total = self.product_to_total.get(product).unwrap_or(&0.0);

        self.product_to_total.insert(product.to_owned(), current_total + price);
    }

    pub fn product_total(&self, product: &str) -> f64 {
        self.product_to_total.get(product).unwrap_or(&0.0).to_owned()
    }
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
}
