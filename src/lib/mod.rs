use std::collections::HashMap;

pub mod loader;

pub type Price = f64;
type Product = String;
type Category = String;

#[derive(Clone, Debug)]
pub struct Finance {
    logs: Vec<(String, Price)>,
    product_categories: HashMap<Product, Category>,
}

impl Finance {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            product_categories: HashMap::new(),
        }
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

    pub fn with_product(self, product: &str, category: &str) -> Self {
        Self {
            product_categories: self
                .product_categories
                .into_iter()
                .chain(vec![(product.to_owned(), category.to_owned())])
                .collect(),
            ..self
        }
    }
}

pub struct ExpenditureLogStats {
    log: Finance,
}

impl ExpenditureLogStats {
    pub fn new(log: Finance) -> Self {
        Self { log }
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
        let mut product_totals = HashMap::<String, Price>::new();
        for (product, price) in self.log.logs.iter() {
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
            .map(|(product, price)| {
                if let Some(c) = self.log.product_categories.get(product) {
                    if c == category {
                        return price.to_owned();
                    }
                }

                return 0.0;
            })
            .sum()
    }

    pub fn category_totals(&self) -> HashMap<String, Price> {
        let mut category_totals = HashMap::<String, Price>::new();

        for (product, price) in self.log.logs.iter() {
            if let Some(category) = self.log.product_categories.get(product) {
                if let Some(current_price) = category_totals.get(category) {
                    category_totals.insert(category.to_owned(), current_price + price);
                }

                category_totals.insert(category.to_owned(), price.to_owned());
            }
        }

        category_totals
    }
}

#[cfg(test)]
mod tests {
    use super::{ExpenditureLogStats, Finance};

    #[test]
    fn expenditure_log_product_total() {
        let log = Finance::new().with_log("prod1", 10.0);
        let stats = ExpenditureLogStats::new(log);

        assert_eq!(stats.product_total("prod1"), 10.0);
    }

    #[test]
    fn expenditure_log_category_total() {
        let log = Finance::new()
            .with_product("prod1", "cat1")
            .with_log("prod1", 10.0);
        let expenditure_log = ExpenditureLogStats::new(log);

        assert_eq!(expenditure_log.category_total("cat1"), 10.0);
    }

    #[test]
    fn product_totals() {
        let log = Finance::new().with_log("prod1", 10.0);
        let expenditure_log = ExpenditureLogStats::new(log);

        assert_eq!(expenditure_log.product_totals().get("prod1"), Some(&10.0));
    }

    #[test]
    fn category_totals() {
        let log = Finance::new()
            .with_product("prod1", "cat1")
            .with_log("prod1", 10.0);
        let expenditure_log = ExpenditureLogStats::new(log);

        assert_eq!(expenditure_log.category_totals().get("cat1"), Some(&10.0));
    }
}
