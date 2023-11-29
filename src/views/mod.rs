use std::collections::HashMap;

use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, ListView, TextView},
};

use crate::siv::{get_finance_app, set_finance_app};
use neo_budget::Price;

pub fn add_log_view() -> Dialog {
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
                .parse::<Price>();

            let mut add_log_result = siv
                .find_name::<TextView>("add_log_result")
                .expect("Couldn't find add_log_result");

            let current_app = get_finance_app(siv);
            let current_log = current_app.finance();

            match add_log_price {
                Ok(price) => {
                    set_finance_app(
                        siv,
                        current_app.with_finance(current_log.with_log(&add_log_name, price)),
                    );
                    add_log_result.set_content("Log added successfully!");
                }

                Err(_) => {
                    add_log_result.set_content("Invalid number!");
                }
            }
        })
        .button("Back", |siv| {
            siv.pop_layer();
        })
        .h_align(cursive::align::HAlign::Center)
}

pub fn view_totals_view(totals: &HashMap<String, Price>) -> Dialog {
    let mut list_view = ListView::new();

    for (product, total) in totals {
        list_view.add_child(product, TextView::new(format!("{total:.2}")));
    }

    Dialog::around(list_view).button("Back", |siv| {
        siv.pop_layer();
    })
}

pub fn save_view() -> Dialog {
    Dialog::around(TextView::new("Saved!")).button("Ok", |siv| {
        siv.pop_layer();
    })
}
