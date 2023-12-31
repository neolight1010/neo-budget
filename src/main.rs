use neo_budget::repository::{EnvJSONFinanceRepository, FinanceRepository};
use views::main_menu::main_menu_view;

use crate::siv::FinanceApp;

mod siv;
mod views;

fn main() -> Result<(), String> {
    let finance_repo = EnvJSONFinanceRepository::from_env()?;
    let finance = finance_repo.load()?;

    let finance_app = FinanceApp::new(finance_repo, finance);

    let mut siv = cursive::default();
    siv.set_user_data(finance_app);
    siv.set_global_callback('q', |siv| {
        let popped_layer = siv.pop_layer();

        if popped_layer.is_none() {
            siv.quit();
        }
    });

    siv.add_layer(main_menu_view());

    siv.run();

    Ok(())
}
