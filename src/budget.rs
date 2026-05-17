use crate::category::Category;
use crate::date::Month;
use crate::error::{parse_error, Error, Result};
use crate::money::Money;
use crate::txn::{Kind, Transaction};

/// A monthly spending limit for a category (and everything nested under it).
#[derive(Debug, Clone, PartialEq)]
pub struct Budget {
    pub category: Category,
    pub limit: Money,
}
/// How a budget stands for one month.
#[derive(Debug, Clone, PartialEq)]
pub struct BudgetStatus {
    pub budget: Budget,
    pub spent: Money,
    pub remaining: Money,
    /// Share of the limit used, in basis points; `None` for a zero limit.
    pub used_bp: Option<i64>,
}

impl BudgetStatus {
    pub fn is_over(&self) -> bool {
        self.remaining.is_negative()
    }
}
