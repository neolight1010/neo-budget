use std::collections::HashMap;

use cursive::views::{Panel, SelectView};
use neo_budget::repository::{EnvJSONFinanceRepository, FinanceRepository};
use neo_budget::stats::FinanceStats;
use siv::get_finance_app;
use views::show_logs::show_labeled_logs_view;

use crate::siv::FinanceApp;
use crate::views::{add_log_view, save_view};

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
        let product_totals = stats.product_totals();
        let category_totals = stats.category_totals();

        match selection {
            MenuSelection::AddLog => {
                siv.add_layer(add_log_view());
            }

            MenuSelection::ViewProductTotals => {
                let mut labeled_logs = HashMap::new();
                labeled_logs.insert("label".to_owned(), product_totals.clone());
                labeled_logs.insert("label2".to_owned(), product_totals.clone());

                siv.add_layer(show_labeled_logs_view(labeled_logs.clone()));
            }

            MenuSelection::ViewCategoryTotals => {
                let mut labeled_logs = HashMap::new();
                labeled_logs.insert("label".to_owned(), category_totals);

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
