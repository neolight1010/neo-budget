use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, Panel, SelectView, TextView},
};
use neo_budget::ExpenditureLog;

enum MenuSelection {
    AddLog,
}

fn main() {
    let mut siv = cursive::default();
    siv.set_user_data(ExpenditureLog::new());

    siv.add_layer(Panel::new(menu_view()));

    siv.run();
}

fn menu_view() -> SelectView<MenuSelection> {
    let mut menu = SelectView::<MenuSelection>::new();
    menu.add_item("Add log", MenuSelection::AddLog);

    menu.set_on_submit(|siv, selection| {
        siv.pop_layer();

        match selection {
            MenuSelection::AddLog => {
                siv.add_layer(add_log_view());
            }
        }
    });

    menu
}

fn add_log_view() -> Dialog {
    let layout = LinearLayout::new(cursive::direction::Orientation::Vertical)
        .child(TextView::new("Product name"))
        .child(EditView::new().with_name("add_log_name").fixed_width(20))
        .child(TextView::new("Price"))
        .child(EditView::new().with_name("add_log_price").fixed_width(20))
        .child(TextView::empty().with_name("add_log_result"));

    Dialog::around(layout)
        .button("Submit", |siv| {
            let add_log_name = siv
                .find_name::<EditView>("add_log_name")
                .expect("Couldn't find add_log_name")
                .get_content();

            let add_log_price = siv
                .find_name::<EditView>("add_log_price")
                .expect("Couldn't find add_log_price")
                .get_content()
                .parse::<f64>();

            let mut add_log_result = siv
                .find_name::<TextView>("add_log_result")
                .expect("Couldn't find add_log_result");

            match add_log_price {
                Ok(price) => {
                    siv.with_user_data(|expenditure_log: &mut ExpenditureLog| {
                        expenditure_log.add_log(&add_log_name, price);
                    });

                    add_log_result.set_content("Log added successfully!");
                }

                Err(_) => {
                    add_log_result.set_content("Invalid number!");
                }
            }
        })
        .h_align(cursive::align::HAlign::Center)
}
