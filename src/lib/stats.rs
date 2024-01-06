use std::collections::HashMap;

use gregorian::YearMonth;

use super::finance::{Finance, Price};

pub struct FinanceStats {
    finance: Finance,
}

pub type LabeledTotals = HashMap<String, Price>;

impl FinanceStats {
    pub fn new(log: Finance) -> Self {
        Self { finance: log }
    }

    pub fn product_totals_by_year_month(&self) -> HashMap<YearMonth, LabeledTotals> {
        self.group_logs_by_year_month_and_label(|product| Some(product.to_owned()))
    }

    pub fn category_totals_by_year_month(&self) -> HashMap<YearMonth, LabeledTotals> {
        self.group_logs_by_year_month_and_label(|product| self.finance.get_category_for(product))
    }

    fn group_logs_by_year_month_and_label(
        &self,
        label_fn: impl Fn(&str) -> Option<String>,
    ) -> HashMap<YearMonth, LabeledTotals> {
        let mut result = HashMap::<YearMonth, LabeledTotals>::new();

        for log in &self.finance.logs {
            let year_month_map = result.entry(log.year_month).or_default();
            let label = label_fn(&log.product);

            if let Some(label) = label {
                let current_total = year_month_map
                    .entry(label.clone())
                    .or_insert(0.0)
                    .to_owned();

                let new_total = current_total + log.price;

                year_month_map.insert(label.clone(), new_total);
            }
        }

        result
    }

    pub fn category_totals(&self) -> HashMap<String, Price> {
        let mut category_totals = HashMap::<String, Price>::new();

        for log in self.finance.logs.iter() {
            if let Some(category) = self.finance.get_category_for(&log.product) {
                if let Some(current_price) = category_totals.get(&category) {
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
    use gregorian::{Month, YearMonth};

    use crate::finance::FinanceLog;

    use super::{Finance, FinanceStats};

    #[test]
    fn product_totals_by_year_month() {
        let finance = Finance::new()
            .with_log(FinanceLog::new(
                "prod1",
                10.0,
                YearMonth::new(2021, Month::January),
            ))
            .with_log(FinanceLog::new(
                "prod2",
                20.0,
                YearMonth::new(2022, Month::February),
            ));

        let stats = FinanceStats::new(finance);

        let totals_by_year_month = stats.product_totals_by_year_month();

        assert_eq!(
            totals_by_year_month
                .get(&YearMonth::new(2021, Month::January))
                .unwrap()
                .get("prod1")
                .unwrap()
                .to_owned(),
            10.0
        );

        assert_eq!(
            totals_by_year_month
                .get(&YearMonth::new(2022, Month::February))
                .unwrap()
                .get("prod2")
                .unwrap()
                .to_owned(),
            20.0
        );
    }

    #[test]
    fn category_totals_by_year_month() {
        let finance = Finance::new()
            .with_log(FinanceLog::new(
                "prod1",
                10.0,
                YearMonth::new(2021, Month::January),
            ))
            .with_log(FinanceLog::new(
                "prod2",
                20.0,
                YearMonth::new(2022, Month::February),
            ))
            .with_product("prod1", "cat1")
            .with_product("prod2", "cat2");

        let stats = FinanceStats::new(finance);

        let totals_by_year_month = stats.category_totals_by_year_month();

        println!("{:?}", totals_by_year_month);

        assert_eq!(
            totals_by_year_month
                .get(&YearMonth::new(2021, Month::January))
                .unwrap()
                .get("cat1")
                .unwrap()
                .to_owned(),
            10.0
        );

        assert_eq!(
            totals_by_year_month
                .get(&YearMonth::new(2022, Month::February))
                .unwrap()
                .get("cat2")
                .unwrap()
                .to_owned(),
            20.0
        );
    }

    #[test]
    fn category_totals() {
        let log = Finance::new()
            .with_product("prod1", "cat1")
            .with_log(simple_log());
        let expenditure_log = FinanceStats::new(log);

        assert_eq!(expenditure_log.category_totals().get("cat1"), Some(&10.0));
    }

    fn simple_log() -> FinanceLog {
        FinanceLog::new("prod1", 10.0, YearMonth::new(2021, Month::January))
    }
}
