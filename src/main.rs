use cursive::views::{Panel, SelectView};
use neo_budget::ExpenditureLog;
use views::{add_log_view, view_totals_view};

mod views;

enum MenuSelection {
    AddLog,
    ViewProductTotals,
    ViewCategoryTotals,
}

fn main() {
    let mut log = ExpenditureLog::new();
    log.add_product("Bread", "Food");
    log.add_product("Eggs", "Food");
    log.add_log("Bread", 10.0);
    log.add_log("Eggs", 15.0);

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
            .user_data::<ExpenditureLog>()
            .expect("Couldn't find expenditure log.");

        let product_totals = expenditure_log.product_totals();
        let category_totals = expenditure_log.category_totals();

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
