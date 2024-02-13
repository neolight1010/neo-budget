use std::collections::HashMap;

use gregorian::YearMonth;

pub type Price = f64;
type ProductName = String;
type ProductId = String;
type Category = String;

#[derive(Clone, Debug)]
pub struct Finance {
    pub logs: Vec<FinanceLog>,
    products: HashMap<ProductId, Product>,
}

impl Finance {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            products: HashMap::new(),
        }
    }

    pub fn products(&self) -> HashMap<ProductId, Product> {
        self.products.clone()
    }

    pub fn with_log(self, log: FinanceLog) -> Self {
        Self {
            logs: self.logs.into_iter().chain(Some(log)).collect(),
            ..self
        }
    }

    pub fn with_product(self, product: &Product) -> Self {
        Self {
            products: self
                .products
                .into_iter()
                .chain(vec![(product.id.to_owned(), product.clone())])
                .collect(),
            ..self
        }
    }

    pub fn get_category_for(&self, product_id: &str) -> Option<Category> {
        self.products.get(product_id).map(|p| p.category.clone())
    }
}

impl Default for Finance {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Product {
    // TODO Add name to product
    id: ProductId,
    pub category: Category,
}

impl Product {
    pub fn new(id: &str, category: &str) -> Self {
        Self {
            id: id.to_owned(),
            category: category.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinanceLog {
    pub product: ProductName,
    pub price: Price,
    pub year_month: YearMonth,
}

impl FinanceLog {
    pub fn new(product: &str, price: f64, year_month: YearMonth) -> Self {
        FinanceLog {
            product: product.to_owned(),
            price,
            year_month,
        }
    }
}
