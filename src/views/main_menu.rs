use std::collections::HashMap;

use cursive::view::IntoBoxedView;
use cursive::views::{Panel, SelectView};
use cursive::View;
use gregorian::YearMonth;
use neo_budget::repository::FinanceRepository;
use neo_budget::stats::{FinanceStats, LabeledTotals};

use super::add_logs::add_log_view;
use super::add_products::add_products_view;
use super::save::save_view;
use super::show_logs::show_labeled_logs_view;
use crate::siv::get_finance_app;

enum MenuSelection {
    AddLog,
    AddProducts,
    ViewProductTotals,
    ViewCategoryTotals,
    Save,
}

pub fn main_menu_view() -> Box<dyn View> {
    Panel::new(
        SelectView::<MenuSelection>::new()
            .item("Add log", MenuSelection::AddLog)
            .item("Add products", MenuSelection::AddProducts)
            .item("Product totals", MenuSelection::ViewProductTotals)
            .item("Category totals", MenuSelection::ViewCategoryTotals)
            .item("Save", MenuSelection::Save)
            .on_submit(|siv, selection| {
                let finance_app = get_finance_app(siv);

                let finance = finance_app.finance();
                let finance_repo = finance_app.finance_repo();

                let stats = FinanceStats::new(finance.clone());

                match selection {
                    MenuSelection::AddLog => {
                        siv.add_layer(add_log_view());
                    }

                    MenuSelection::AddProducts => {
                        siv.add_layer(add_products_view());
                    }

                    MenuSelection::ViewProductTotals => {
                        let labeled_logs =
                            year_month_totals_display(stats.product_totals_by_year_month());
                        siv.add_layer(show_labeled_logs_view(labeled_logs.clone()));
                    }

                    MenuSelection::ViewCategoryTotals => {
                        let labeled_logs =
                            year_month_totals_display(stats.category_totals_by_year_month());
                        siv.add_layer(show_labeled_logs_view(labeled_logs.clone()));
                    }

                    MenuSelection::Save => {
                        finance_repo.save(&finance).unwrap(); // TODO handle error
                        siv.add_layer(save_view());
                    }
                }
            }),
    )
    .into_boxed_view()
}

fn year_month_totals_display(
    labeled_totals_by_year_month: HashMap<YearMonth, LabeledTotals>,
) -> HashMap<String, LabeledTotals> {
    let mut labeled_logs = HashMap::new();
    for (year_month, product_totals) in labeled_totals_by_year_month {
        labeled_logs.insert(year_month.to_string(), product_totals);
    }

    labeled_logs
}
