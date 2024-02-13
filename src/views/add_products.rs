use cursive::{
    view::Nameable,
    views::{Dialog, EditView, LinearLayout, TextView, ViewRef},
    Cursive, With,
};
use neo_budget::finance::Product;

use crate::siv::update_finance_app;

const PRODUCT_NAME_INPUT_NAME: &str = "product_name";
const CATEGORY_NAME_INPUT_NAME: &str = "category_name";
const INFO_TEXT_NAME: &str = "info_text";

pub fn add_products_view() -> Dialog {
    LinearLayout::vertical()
        .child(product_name_input())
        .child(category_name_input())
        .child(TextView::empty().with_name(INFO_TEXT_NAME))
        .wrap_with(|view| {
            Dialog::around(view)
                .button("Ok", submit_button_action)
                .button("Back", |siv| {
                    siv.pop_layer();
                })
        })
}

fn product_name_input() -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("Product name"))
        .child(EditView::new().with_name(PRODUCT_NAME_INPUT_NAME))
}

fn category_name_input() -> LinearLayout {
    LinearLayout::vertical()
        .child(TextView::new("Category name"))
        .child(EditView::new().with_name(CATEGORY_NAME_INPUT_NAME))
}

fn submit_button_action(siv: &mut Cursive) {
    let update_result = update_finance_app(siv, |siv, app| {
        let product = siv
            .find_name::<EditView>(PRODUCT_NAME_INPUT_NAME)
            .map(|edit_view| edit_view.get_content());

        let category = siv
            .find_name::<EditView>(CATEGORY_NAME_INPUT_NAME)
            .map(|edit_view| edit_view.get_content());

        let current_finance = app.finance();

        match (product, category) {
            (Some(product), Some(category)) => {
                app.with_finance(current_finance.with_product(&Product::new(&product, &category)))
            }

            _ => {
                get_info_text_view(siv).set_content("Error adding product!");

                app
            }
        }
    });

    match update_result {
        Ok(_) => {
            get_info_text_view(siv).set_content("Product added successfully!");
        }

        _ => {
            get_info_text_view(siv).set_content("Error adding product!");
        }
    }
}

fn get_info_text_view(siv: &mut Cursive) -> ViewRef<TextView> {
    siv.find_name::<TextView>(INFO_TEXT_NAME)
        .expect("Info-text view should exist.")
}
