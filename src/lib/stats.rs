use std::collections::HashMap;

use gregorian::YearMonth;

use super::finance::{Finance, Price};

pub struct FinanceStats {
    finance: Finance,
}

#[derive(Clone)]
pub struct GroupedTotals {
    pub labeled: HashMap<String, Price>,
    pub unlabeled: Price,
}

impl Default for GroupedTotals {
    fn default() -> Self {
        Self {
            labeled: HashMap::new(),
            unlabeled: 0.0,
        }
    }
}

impl FinanceStats {
    pub fn new(log: Finance) -> Self {
        Self { finance: log }
    }

    pub fn product_totals_by_year_month(&self) -> HashMap<YearMonth, GroupedTotals> {
        self.group_logs_by_year_month_and_label(|product| Some(product.to_owned()))
    }

    pub fn category_totals_by_year_month(&self) -> HashMap<YearMonth, GroupedTotals> {
        self.group_logs_by_year_month_and_label(|product| self.finance.get_category_for(product))
    }

    fn group_logs_by_year_month_and_label(
        &self,
        label_fn: impl Fn(&str) -> Option<String>,
    ) -> HashMap<YearMonth, GroupedTotals> {
        let mut result = HashMap::<YearMonth, GroupedTotals>::new();

        for log in &self.finance.logs {
            let grouped_totals = result.entry(log.year_month).or_default();
            let label = label_fn(&log.product);

            if let Some(label) = label {
                let current_total = grouped_totals
                    .labeled
                    .entry(label.clone())
                    .or_insert(0.0)
                    .to_owned();

                let new_total = current_total + log.price;

                grouped_totals.labeled.insert(label.clone(), new_total);
            } else {
                grouped_totals.unlabeled += log.price;
            }
        }

        result
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
                .labeled
                .get("prod1")
                .unwrap()
                .to_owned(),
            10.0
        );

        assert_eq!(
            totals_by_year_month
                .get(&YearMonth::new(2022, Month::February))
                .unwrap()
                .labeled
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
            .with_log(FinanceLog::new(
                "prod3",
                30.0,
                YearMonth::new(2022, Month::March),
            ))
            .with_product("prod1", "cat1")
            .with_product("prod2", "cat2");

        let stats = FinanceStats::new(finance);

        let totals_by_year_month = stats.category_totals_by_year_month();

        assert_eq!(
            totals_by_year_month
                .get(&YearMonth::new(2021, Month::January))
                .unwrap()
                .labeled
                .get("cat1"),
            Some(&10.0)
        );

        assert_eq!(
            totals_by_year_month
                .get(&YearMonth::new(2022, Month::February))
                .unwrap()
                .labeled
                .get("cat2"),
            Some(&20.0)
        );

        assert_eq!(
            totals_by_year_month
                .get(&YearMonth::new(2022, Month::March))
                .unwrap()
                .unlabeled,
            30.0
        );
    }
}
