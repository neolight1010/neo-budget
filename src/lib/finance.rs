use std::collections::HashMap;

pub type Price = f64;
type Product = String;
type Category = String;

#[derive(Clone, Debug)]
pub struct Finance {
    pub logs: Vec<(String, Price)>,
    pub product_categories: HashMap<Product, Category>,
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
