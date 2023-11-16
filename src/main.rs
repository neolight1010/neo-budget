use cursive::views::{Panel, SelectView};
use neo_budget::repository::JSONFinanceRepository;
use neo_budget::{ExpenditureLogStats, Finance};
use views::{add_log_view, view_totals_view};

mod views;

enum MenuSelection {
    AddLog,
    ViewProductTotals,
    ViewCategoryTotals,
}

fn main() -> Result<(), String> {
    let finance_repo = JSONFinanceRepository::from_env()?;
    let log = finance_repo.load()?;

    let mut siv = cursive::default();
    siv.set_user_data(log);

    siv.add_layer(Panel::new(menu_view()));

    siv.run();

    Ok(())
}

fn menu_view() -> SelectView<MenuSelection> {
    let mut menu = SelectView::<MenuSelection>::new();
    menu.add_item("Add log", MenuSelection::AddLog);
    menu.add_item("Product totals", MenuSelection::ViewProductTotals);
    menu.add_item("Category totals", MenuSelection::ViewCategoryTotals);

    menu.set_on_submit(|siv, selection| {
        let expenditure_log = siv
            .user_data::<Finance>()
            .expect("Couldn't find expenditure log.");

        let stats = ExpenditureLogStats::new(expenditure_log.clone());
        let product_totals = stats.product_totals();
        let category_totals = stats.category_totals();

        match selection {
            MenuSelection::AddLog => {
                siv.add_layer(add_log_view());
            }

            MenuSelection::ViewProductTotals => {
                siv.add_layer(view_totals_view(&product_totals));
            }

            MenuSelection::ViewCategoryTotals => {
                siv.add_layer(view_totals_view(&category_totals));
            }
        }
    });

    menu
}
