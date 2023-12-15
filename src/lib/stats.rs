use std::collections::HashMap;

use super::finance::{Finance, FinanceLog, Price};

pub struct FinanceStats {
    log: Finance,
}

impl FinanceStats {
    pub fn new(log: Finance) -> Self {
        Self { log }
    }

    pub fn product_total(&self, product: &str) -> Price {
        self.log
            .logs
            .iter()
            .filter(|log| log.product == product)
            .map(|log| log.price)
            .sum()
    }

    pub fn product_totals(&self) -> HashMap<String, Price> {
        let mut product_totals = HashMap::<String, Price>::new();
        for FinanceLog { product, price, .. } in self.log.logs.iter() {
            if let Some(current_price) = product_totals.get(product) {
                product_totals.insert(product.to_owned(), current_price + price);
            } else {
                product_totals.insert(product.to_owned(), price.to_owned());
            }
        }

        product_totals
    }

    pub fn category_total(&self, category: &str) -> Price {
        self.log
            .logs
            .iter()
            .map(|log| {
                if let Some(c) = self.log.product_categories.get(&log.product) {
                    if c == category {
                        return log.price;
                    }
                }

                0.0
            })
            .sum()
    }

    pub fn category_totals(&self) -> HashMap<String, Price> {
        let mut category_totals = HashMap::<String, Price>::new();

        for log in self.log.logs.iter() {
            if let Some(category) = self.log.product_categories.get(&log.product) {
                if let Some(current_price) = category_totals.get(category) {
                    category_totals.insert(category.to_owned(), current_price + log.price);
                }

                category_totals.insert(category.to_owned(), log.price);
            }
        }

        category_totals
    }
}

#[cfg(test)]
mod tests {
    use super::{Finance, FinanceStats};

    #[test]
    fn expenditure_log_product_total() {
        let log = Finance::new().with_log("prod1", 10.0);
        let stats = FinanceStats::new(log);

        assert_eq!(stats.product_total("prod1"), 10.0);
    }

    #[test]
    fn expenditure_log_category_total() {
        let log = Finance::new()
            .with_product("prod1", "cat1")
            .with_log("prod1", 10.0);
        let expenditure_log = FinanceStats::new(log);

        assert_eq!(expenditure_log.category_total("cat1"), 10.0);
    }

    #[test]
    fn product_totals() {
        let log = Finance::new().with_log("prod1", 10.0);
        let expenditure_log = FinanceStats::new(log);

        assert_eq!(expenditure_log.product_totals().get("prod1"), Some(&10.0));
    }

    #[test]
    fn category_totals() {
        let log = Finance::new()
            .with_product("prod1", "cat1")
            .with_log("prod1", 10.0);
        let expenditure_log = FinanceStats::new(log);

        assert_eq!(expenditure_log.category_totals().get("cat1"), Some(&10.0));
    }
}
