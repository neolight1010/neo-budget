use std::collections::HashMap;

use cursive::{
    view::Nameable,
    views::{Dialog, LinearLayout, ListView, Panel, SelectView, TextView},
};
use neo_budget::stats::GroupedTotals;

type LogCollection = HashMap<String, GroupedTotals>;

pub fn show_grouped_totals_view(log_collection: LogCollection) -> Dialog {
    const LOG_LIST_VIEW_NAME: &str = "log_list";

    let mut log_list = ListView::new();
    reload_logs_list(
        &mut log_list,
        log_collection.values().next().unwrap().clone(),
    );

    let mut label_select_view = SelectView::new();
    for label in log_collection.keys() {
        label_select_view.add_item(label.clone(), label.clone());
    }

    label_select_view.set_on_select(move |siv, selected_label| {
        let mut log_list = siv
            .find_name::<ListView>(LOG_LIST_VIEW_NAME)
            .expect("Couldn't find log_list view");

        reload_logs_list(
            &mut log_list,
            log_collection
                .get(selected_label)
                .cloned()
                .unwrap_or_default()
                .clone()
        );
    });

    Dialog::around(
        LinearLayout::horizontal()
            .child(Panel::new(label_select_view))
            .child(Panel::new(log_list.with_name(LOG_LIST_VIEW_NAME))),
    )
    .button("Back", |siv| {
        siv.pop_layer();
    })
}

fn reload_logs_list(log_list: &mut ListView, log_collection: GroupedTotals) {
    log_list.clear();

    for (product, total) in &log_collection.labeled {
        log_list.add_child(product, TextView::new(format!("{total:.2}")));
    }
}
