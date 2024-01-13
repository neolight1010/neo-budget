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
    reload_logs_list(&mut log_list, log_collection.values().next().cloned());

    let mut label_select_view = SelectView::new();
    for label in log_collection.keys() {
        label_select_view.add_item(label.clone(), label.clone());
    }

    label_select_view.set_on_select(move |siv, selected_label| {
        let mut log_list = siv
            .find_name::<ListView>(LOG_LIST_VIEW_NAME)
            .expect("Couldn't find log_list view");

        reload_logs_list(&mut log_list, log_collection.get(selected_label).cloned());
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

fn reload_logs_list(log_list: &mut ListView, log_collection: Option<GroupedTotals>) {
    log_list.clear();

    if let Some(log_collection) = log_collection {
        for (product, total) in &log_collection.labeled {
            add_item_to_log_list(log_list, product, *total);
        }

        if log_collection.unlabeled > 0.0 {
            add_item_to_log_list(log_list, "<others", log_collection.unlabeled);
        }
    }
}

fn add_item_to_log_list(log_list: &mut ListView, label: &str, total: f64) {
    log_list.add_child(label, TextView::new(format!("{total:.2}")));
}
