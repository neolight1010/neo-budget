use cursive::views::{Dialog, TextView};

pub fn save_view() -> Dialog {
    Dialog::around(TextView::new("Saved!")).button("Ok", |siv| {
        siv.pop_layer();
    })
}
