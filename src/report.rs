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
/// One [`Totals`] per currency, alphabetically.
pub fn totals(transactions: &[&Transaction]) -> Result<Vec<Totals>> {
    let mut by_currency: BTreeMap<String, Totals> = BTreeMap::new();
    for t in transactions {
        let currency = t.amount.currency();
        if !by_currency.contains_key(currency) {
            by_currency.insert(currency.to_string(), Totals::empty(currency)?);
        }
        by_currency
            .get_mut(currency)
            .expect("inserted above")
            .record(t)?;
    }
    Ok(by_currency.into_values().collect())
}
use crate::category::Category;
use std::collections::btree_map::Entry;

/// One line of a category breakdown.
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryTotal {
    pub category: Category,
    pub total: Money,
    pub count: usize,
}

/// Totals of `kind` per category, optionally rolled up to `depth` levels.
/// Sorted by currency, then largest first.
pub fn by_category(
    transactions: &[&Transaction],
    kind: Kind,
    depth: Option<usize>,
) -> Result<Vec<CategoryTotal>> {
    let mut groups: BTreeMap<(String, Category), (Money, usize)> = BTreeMap::new();
    for t in transactions.iter().filter(|t| t.kind == kind) {
        let category = match depth {
            Some(d) => t.category.truncate(d),
            None => t.category.clone(),
        };
        let key = (t.amount.currency().to_string(), category);
        let entry = match groups.entry(key) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert((Money::zero(t.amount.currency())?, 0)),
        };
        entry.0 = entry.0.checked_add(&t.amount)?;
        entry.1 += 1;
    }
    let mut rows: Vec<CategoryTotal> = groups
        .into_iter()
        .map(|((_, category), (total, count))| CategoryTotal {
            category,
            total,
            count,
        })
        .collect();
    rows.sort_by(|a, b| {
        a.total
            .currency()
            .cmp(b.total.currency())
            .then(b.total.minor().cmp(&a.total.minor()))
            .then(a.category.cmp(&b.category))
    });
    Ok(rows)
}
