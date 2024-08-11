// TODO: Define a base class named `Account`, with a floating point `balance` property.
//  Then define a subclass named `AccountWithHistory`.
//  `AccountWithHistory` adds a `history` attribute: every time the `balance` is modified,
//  the old balance is stored in the `history` list. `history` can be accessed but not modified
//  directly. The `history` list should be initialized as an empty list.
use pyo3::prelude::*;

#[pyclass(subclass)]
struct Account {
    balance: f64,
}

#[pymethods]
impl Account {
    #[new]
    fn new(balance: f64) -> Self {
        Account { balance }
    }

    #[getter]
    fn balance(&self) -> f64 {
        self.balance
    }

    #[setter]
    fn set_balance(&mut self, balance: f64) {
        self.balance = balance;
    }
}

#[pyclass(extends=Account)]
struct AccountWithHistory {
    history: Vec<f64>,
}

#[pymethods]
impl AccountWithHistory {
    #[new]
    fn new(balance: f64) -> PyClassInitializer<Self> {
        let account = Account::new(balance);
        let mut account_with_history = AccountWithHistory {
            history: Vec::new(),
        };
        PyClassInitializer::from(account).add_subclass(account_with_history)
    }

    #[getter]
    fn history(&self) -> Vec<f64> {
        self.history.clone()
    }

    #[getter]
    fn balance(self_: PyRef<'_, Self>) -> f64 {
        self_.as_super().balance
    }

    #[setter]
    fn set_balance(mut self_: PyRefMut<'_, Self>, balance: f64) {
        let current_balance = self_.as_super().balance;
        self_.history.push(current_balance);
        let account = self_.as_super();
        account.balance = balance;
    }
}

#[pymodule]
fn parent(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Account>()?;
    m.add_class::<AccountWithHistory>()?;
    Ok(())
}
