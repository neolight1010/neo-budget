use std::collections::HashMap;

use cursive::views::{Panel, SelectView};
use gregorian::YearMonth;
use neo_budget::repository::{EnvJSONFinanceRepository, FinanceRepository};
use neo_budget::stats::{FinanceStats, LabeledTotals};
use siv::get_finance_app;

use views::add_logs::add_log_view;
use views::save::save_view;
use views::show_logs::show_labeled_logs_view;

use crate::siv::FinanceApp;

mod siv;
mod views;

enum MenuSelection {
    AddLog,
    ViewProductTotals,
    ViewCategoryTotals,
    Save,
}

fn main() -> Result<(), String> {
    let finance_repo = EnvJSONFinanceRepository::from_env()?;
    let finance = finance_repo.load()?;

    let finance_app = FinanceApp::new(finance_repo, finance);

    let mut siv = cursive::default();
    siv.set_user_data(finance_app);

    siv.add_layer(Panel::new(menu_view()));

    siv.run();

    Ok(())
}

fn menu_view() -> SelectView<MenuSelection> {
    let mut menu = SelectView::<MenuSelection>::new();
    menu.add_item("Add log", MenuSelection::AddLog);
    menu.add_item("Product totals", MenuSelection::ViewProductTotals);
    menu.add_item("Category totals", MenuSelection::ViewCategoryTotals);
    menu.add_item("Save", MenuSelection::Save);

    menu.set_on_submit(|siv, selection| {
        let finance_app = get_finance_app(siv);

        let finance = finance_app.finance();
        let finance_repo = finance_app.finance_repo();

        let stats = FinanceStats::new(finance.clone());

        match selection {
            MenuSelection::AddLog => {
                siv.add_layer(add_log_view());
            }

            MenuSelection::ViewProductTotals => {
                let labeled_logs = year_month_totals_display(stats.product_totals_by_year_month());
                siv.add_layer(show_labeled_logs_view(labeled_logs.clone()));
            }

            MenuSelection::ViewCategoryTotals => {
                let labeled_logs = year_month_totals_display(stats.category_totals_by_year_month());
                siv.add_layer(show_labeled_logs_view(labeled_logs.clone()));
            }

            MenuSelection::Save => {
                finance_repo.save(&finance).unwrap(); // TODO handle error
                siv.add_layer(save_view());
            }
        }
    });

    menu
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
