use cursive::Cursive;
use neo_budget::finance::Finance;
use neo_budget::repository::{EnvJSONFinanceRepository, FinanceRepository};

#[derive(Clone)]
pub struct FinanceApp<T: FinanceRepository> {
    finance: Finance,
    finance_repo: T,
}

impl<T: FinanceRepository> FinanceApp<T> {
    pub fn new(finance_repo: T, finance: Finance) -> Self {
        Self {
            finance_repo,
            finance,
        }
    }

    pub fn with_finance(self, finance: Finance) -> Self {
        Self { finance, ..self }
    }

    pub fn finance(&self) -> Finance {
        self.finance.clone()
    }

    pub fn finance_repo(&self) -> T {
        self.finance_repo.clone()
    }
}

type ChosenFinanceRepository = EnvJSONFinanceRepository;

pub fn update_finance_app(
    siv: &mut cursive::Cursive,
    update: impl Fn(
        &mut Cursive,
        FinanceApp<ChosenFinanceRepository>,
    ) -> FinanceApp<ChosenFinanceRepository>,
) -> Result<(), ()> {
    let current_app = get_finance_app(siv);
    let new_app = update(siv, current_app);
    set_finance_app(siv, new_app);

    Ok(())
}

pub fn get_finance_app(siv: &mut cursive::Cursive) -> FinanceApp<ChosenFinanceRepository> {
    siv.user_data::<FinanceApp<ChosenFinanceRepository>>()
        .unwrap()
        .clone()
}

pub fn set_finance_app(siv: &mut cursive::Cursive, app: FinanceApp<ChosenFinanceRepository>) {
    siv.set_user_data(app);
}
