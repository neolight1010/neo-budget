use cursive::views::{Panel, SelectView};
use neo_budget::ExpenditureLog;
use views::{add_log_view, view_product_totals_view};

mod views;

enum MenuSelection {
    AddLog,
    ViewProductTotals,
}

fn main() {
    let mut log = ExpenditureLog::new();
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

    menu.set_on_submit(|siv, selection| {
        let product_to_total = siv
            .user_data::<ExpenditureLog>()
            .expect("Couldn't find expenditure log.")
            .product_to_total
            .clone();

        siv.pop_layer();

        match selection {
            MenuSelection::AddLog => {
                siv.add_layer(add_log_view());
            }

            MenuSelection::ViewProductTotals => {
                siv.add_layer(view_product_totals_view(&product_to_total));
            }
        }
    });

    menu
}
