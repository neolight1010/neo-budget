use cursive::{
    views::{Dialog, TextView},
    Cursive,
};
use neo_budget::repository::{
    EnvJSONFinanceRepository, FinanceRepository, FinanceRepositoryLoadError,
};
use views::main_menu::main_menu_view;

use crate::siv::FinanceApp;

mod siv;
mod views;

fn main() -> Result<(), String> {
    let finance_repo = EnvJSONFinanceRepository::from_env()?;
    let finance = finance_repo.load();

    let mut siv = cursive::default();
    set_back_button(&mut siv);

    match finance {
        Ok(finance) => {
            let finance_app = FinanceApp::new(finance_repo, finance);
            siv.set_user_data(finance_app);

            siv.add_layer(main_menu_view());
        }

        Err(error) => siv.add_layer(finance_load_error_view(error)),
    }

    siv.run();

    Ok(())
}

fn set_back_button(siv: &mut Cursive) {
    siv.set_global_callback('q', |siv| {
        if siv.pop_layer().is_none() {
            siv.quit();
        }
    });
}

fn finance_load_error_view(error: FinanceRepositoryLoadError) -> Dialog {
    Dialog::around(TextView::new(String::from(error)))
}
