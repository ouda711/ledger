use crate::category::Category;
use crate::date::{Date, Month};
use crate::txn::{Kind, Transaction};

/// Criteria for selecting transactions. Every field that is set must match.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Filter {
    pub from: Option<Date>,
    pub to: Option<Date>,
    pub category: Option<Category>,
    pub kind: Option<Kind>,
    pub tag: Option<String>,
    pub text: Option<String>,
    pub currency: Option<String>,
}
impl Filter {
    pub fn matches(&self, t: &Transaction) -> bool {
        if self.from.is_some_and(|from| t.date < from) {
            return false;
        }
        if self.to.is_some_and(|to| t.date > to) {
            return false;
        }
        if let Some(category) = &self.category {
            if !t.category.is_within(category) {
                return false;
            }
        }
        if self.kind.is_some_and(|kind| t.kind != kind) {
            return false;
        }
        if let Some(tag) = &self.tag {
            if !t.has_tag(tag) {
                return false;
            }
        }
        if let Some(currency) = &self.currency {
            if !t.amount.currency().eq_ignore_ascii_case(currency) {
                return false;
            }
        }
        if let Some(text) = &self.text {
            let needle = text.to_lowercase();
            if !t.note.to_lowercase().contains(&needle) && !t.category.as_str().contains(&needle) {
                return false;
            }
        }
        true
    }
}
/// The transactions matching `filter`, in ledger order.
pub fn select<'a>(transactions: &'a [Transaction], filter: &Filter) -> Vec<&'a Transaction> {
    transactions.iter().filter(|t| filter.matches(t)).collect()
}
impl Filter {
    /// Everything dated within `month`.
    pub fn for_month(month: Month) -> Filter {
        Filter {
            from: Some(month.first_day()),
            to: Some(month.last_day()),
            ..Filter::default()
        }
    }

    /// True when no criteria are set.
    pub fn is_empty(&self) -> bool {
        *self == Filter::default()
    }
}
