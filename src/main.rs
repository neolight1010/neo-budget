use cursive::views::{Panel, SelectView};
use neo_budget::{Finance, ExpenditureLogStats};
use views::{add_log_view, view_totals_view};

mod views;

enum MenuSelection {
    AddLog,
    ViewProductTotals,
    ViewCategoryTotals,
}

fn main() {
    let log = Finance::new()
        .with_product("Bread", "Food")
        .with_product("Eggs", "Food")
        .with_log("Bread", 10.0)
        .with_log("Eggs", 15.0);

    let mut siv = cursive::default();
    siv.set_user_data(log);

    siv.add_layer(Panel::new(menu_view()));

    siv.run();
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
