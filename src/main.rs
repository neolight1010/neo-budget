use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, Panel, SelectView, TextView},
};

enum MenuSelection {
    AddLog,
}

fn main() {
    let mut siv = cursive::default();

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
        .child(EditView::new().fixed_width(20))
        .child(TextView::new("Price"))
        .child(EditView::new().fixed_width(20))
        .child(TextView::empty().with_name("add_log_result"));

    Dialog::around(layout)
        .button("Submit", |siv| {
            let mut add_log_result = siv
                .find_name::<TextView>("add_log_result")
                .expect("Couldn't find add_log_result");

            add_log_result.set_content("Log added successfully!");
        })
        .h_align(cursive::align::HAlign::Center)
}
