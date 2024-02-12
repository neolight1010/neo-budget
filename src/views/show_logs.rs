use std::collections::HashMap;

use cursive::{
    view::Nameable,
    views::{Dialog, LinearLayout, ListView, Panel, SelectView, TextView},
};
use im::Vector;
use itertools::Itertools;
use neo_budget::stats::GroupedTotals;

type LogCollection = HashMap<String, GroupedTotals>;

pub fn show_grouped_totals_view(log_collection: LogCollection) -> Dialog {
    let mut log_list = ListView::new();
    reload_logs_list(&mut log_list, log_collection.values().next().cloned());

    let (label_select_view, label_select_view_name) = build_select_view(log_collection);

    Dialog::around(
        LinearLayout::horizontal()
            .child(Panel::new(label_select_view))
            .child(Panel::new(log_list.with_name(label_select_view_name))),
    )
    .button("Back", |siv| {
        siv.pop_layer();
    })
}

fn build_select_view(log_collection: HashMap<String, GroupedTotals>) -> (SelectView, String) {
    const LOG_LIST_VIEW_NAME: &str = "log_list";

    let mut label_select_view = SelectView::new();
    for label in log_collection.keys().sorted() {
        label_select_view.add_item(label.clone(), label.clone());
    }

    label_select_view.set_on_select(move |siv, selected_label| {
        let mut log_list = siv
            .find_name::<ListView>(LOG_LIST_VIEW_NAME)
            .expect("Couldn't find log_list view");

        reload_logs_list(&mut log_list, log_collection.get(selected_label).cloned());
    });
    (label_select_view, LOG_LIST_VIEW_NAME.to_owned())
}

fn reload_logs_list(log_list: &mut ListView, log_collection: Option<GroupedTotals>) {
    log_list.clear();

    if let Some(log_collection) = log_collection {
        for (product, total) in sorted_labeled_logs(&log_collection) {
            add_item_to_log_list(log_list, &product, total);
        }

        if log_collection.unlabeled > 0.0 {
            add_item_to_log_list(log_list, "<others>", log_collection.unlabeled);
        }
    }
}

fn sorted_labeled_logs(log_collection: &GroupedTotals) -> Vector<(String, f64)> {
    let mut vectorized = log_collection
        .labeled
        .clone()
        .into_iter()
        .collect::<Vector<(String, f64)>>();

    vectorized.sort_by(|left, right| match left.0 < right.0 {
        true => std::cmp::Ordering::Less,
        false => std::cmp::Ordering::Greater,
    });

    vectorized
}

fn add_item_to_log_list(log_list: &mut ListView, label: &str, total: f64) {
    log_list.add_child(label, TextView::new(format!("{total:.2}")));
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use im::vector;
    use neo_budget::stats::GroupedTotals;

    use super::sorted_labeled_logs;

    #[test]
    fn test_sorted_labeled_logs() {
        let mut labeled = HashMap::new();
        labeled.insert("b".to_owned(), 2.0);
        labeled.insert("c".to_owned(), 3.0);
        labeled.insert("a".to_owned(), 1.0);

        let input = GroupedTotals {
            labeled,
            unlabeled: 0.0,
        };
        let result = sorted_labeled_logs(&input);

        assert_eq!(
            result,
            vector![
                ("a".to_owned(), 1.0),
                ("b".to_owned(), 2.0),
                ("c".to_owned(), 3.0)
            ],
        );
    }
}
