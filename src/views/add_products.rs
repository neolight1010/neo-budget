use cursive::views::{Dialog, TextView};

pub fn add_products_view() -> Dialog {
    Dialog::around(TextView::new("Add Products!")).button("Ok", |siv| {
        siv.pop_layer();
    })
}
