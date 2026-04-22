use crate::error::Result;
use crate::money::Money;
use crate::txn::{Kind, Transaction};
use std::collections::BTreeMap;

/// Income and expense in one currency. Currencies are never added together.
#[derive(Debug, Clone, PartialEq)]
pub struct Totals {
    pub currency: String,
    pub income: Money,
    pub expense: Money,
}

impl Totals {
    fn empty(currency: &str) -> Result<Totals> {
        Ok(Totals {
            currency: currency.to_string(),
            income: Money::zero(currency)?,
            expense: Money::zero(currency)?,
        })
    }

    /// Income minus expense.
    pub fn net(&self) -> Result<Money> {
        self.income.checked_sub(&self.expense)
    }

    fn record(&mut self, t: &Transaction) -> Result<()> {
        match t.kind {
            Kind::Income => self.income = self.income.checked_add(&t.amount)?,
            Kind::Expense => self.expense = self.expense.checked_add(&t.amount)?,
        }
        Ok(())
    }
}
