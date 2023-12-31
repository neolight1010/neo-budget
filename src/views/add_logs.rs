use cursive::{
    view::{Nameable, Resizable},
    views::{Canvas, Dialog, EditView, LinearLayout, TextView},
};
use gregorian::{Month, YearMonth};

use crate::siv::{get_finance_app, set_finance_app};
use neo_budget::finance::{FinanceLog, Price};


pub fn add_log_view() -> Dialog {
    let layout = LinearLayout::new(cursive::direction::Orientation::Vertical)
        .child(TextView::new("Product name"))
        .child(EditView::new().with_name("add_log_name").fixed_width(20))
        .child(TextView::new("Price"))
        .child(EditView::new().with_name("add_log_price").fixed_width(20))
        .child(TextView::new("Year-Month"))
        .child(
            LinearLayout::new(cursive::direction::Orientation::Horizontal)
                .child(EditView::new().with_name("year_input").fixed_width(5))
                .child(Canvas::new(()).fixed_width(2))
                .child(EditView::new().with_name("month_input").fixed_width(3)),
        )
        .child(Canvas::new(()))
        .child(TextView::empty().with_name("add_log_result"));

    Dialog::around(layout)
        .button("Submit", |siv| {
            let add_log_name = siv
                .find_name::<EditView>("add_log_name")
                .expect("Couldn't find add_log_name")
                .get_content();

            let price_input = siv
                .find_name::<EditView>("add_log_price")
                .expect("Couldn't find add_log_price")
                .get_content()
                .parse::<Price>();

            let year_input = siv
                .find_name::<EditView>("year_input")
                .expect("Couldn't find year_input")
                .get_content()
                .parse::<i16>();

            let month_input = siv
                .find_name::<EditView>("month_input")
                .expect("Couldn't find month_input")
                .get_content()
                .parse::<u8>()
                .map(Month::new);

            let mut result_view = siv
                .find_name::<TextView>("add_log_result")
                .expect("Couldn't find add_log_result");

            let current_app = get_finance_app(siv);
            let current_log = current_app.finance();

            match (price_input, year_input, month_input) {
                (Ok(price), Ok(year), Ok(Ok(month))) => {
                    set_finance_app(
                        siv,
                        current_app.with_finance(
                            // TODO Use real date
                            current_log.with_log(FinanceLog::new(
                                &add_log_name,
                                price,
                                YearMonth::new(year, month),
                            )),
                        ),
                    );
                    result_view.set_content("Log added successfully!");
                }

                _ => {
                    result_view.set_content("Invalid input!");
                }
            }
        })
        .button("Back", |siv| {
            siv.pop_layer();
        })
        .h_align(cursive::align::HAlign::Center)
}
