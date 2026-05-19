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
/// Expenses in `month` under the budget's category, in the budget's currency.
pub fn status(budget: &Budget, transactions: &[Transaction], month: Month) -> Result<BudgetStatus> {
    let mut spent = Money::zero(budget.limit.currency())?;
    for t in transactions {
        if t.kind == Kind::Expense
            && month.contains(&t.date)
            && t.category.is_within(&budget.category)
            && t.amount.currency() == budget.limit.currency()
        {
            spent = spent.checked_add(&t.amount)?;
        }
    }
    Ok(BudgetStatus {
        budget: budget.clone(),
        remaining: budget.limit.checked_sub(&spent)?,
        used_bp: spent.ratio_bp(&budget.limit)?,
        spent,
    })
}
/// Every budget, at most one per category and currency.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Budgets {
    items: Vec<Budget>,
}

impl Budgets {
    /// Add a budget, replacing any existing one for the same category and currency.
    pub fn set(&mut self, budget: Budget) -> Result<()> {
        if budget.limit.is_negative() {
            return Err(parse_error("a budget limit cannot be negative"));
        }
        self.items.retain(|b| {
            !(b.category == budget.category && b.limit.currency() == budget.limit.currency())
        });
        self.items.push(budget);
        self.items.sort_by(|a, b| {
            a.category
                .cmp(&b.category)
                .then(a.limit.currency().cmp(b.limit.currency()))
        });
        Ok(())
    }

    /// Remove every budget for `category`.
    pub fn remove(&mut self, category: &Category) -> Result<()> {
        let before = self.items.len();
        self.items.retain(|b| b.category != *category);
        if self.items.len() == before {
            return Err(Error::NotFound(format!("budget for '{category}'")));
        }
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Budget> {
        self.items.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
