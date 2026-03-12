use crate::category::Category;
use crate::date::Date;
use crate::error::{parse_error, Result};
use crate::money::Money;

/// Which way the money moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kind {
    Income,
    Expense,
}

impl Kind {
    pub fn parse(text: &str) -> Result<Kind> {
        match text.trim().to_ascii_lowercase().as_str() {
            "income" | "in" | "i" => Ok(Kind::Income),
            "expense" | "out" | "e" | "ex" => Ok(Kind::Expense),
            other => Err(parse_error(format!(
                "'{other}' is not a kind; use income or expense"
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Income => "income",
            Kind::Expense => "expense",
        }
    }
}
/// One movement of money. `amount` is always positive; `kind` gives the direction.
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id: u64,
    pub date: Date,
    pub kind: Kind,
    pub amount: Money,
    pub category: Category,
    pub note: String,
    pub tags: Vec<String>,
}
fn normalise_tag(tag: &str) -> String {
    tag.trim().trim_start_matches('#').to_lowercase()
}

/// Split `"#trip, Work,trip"` into sorted, de-duplicated tags: `["trip", "work"]`.
pub fn parse_tags(text: &str) -> Vec<String> {
    let mut tags: Vec<String> = text
        .split(',')
        .map(normalise_tag)
        .filter(|t| !t.is_empty())
        .collect();
    tags.sort();
    tags.dedup();
    tags
}
impl Transaction {
    /// Positive for income, negative for expenses.
    pub fn signed(&self) -> Result<Money> {
        match self.kind {
            Kind::Income => Ok(self.amount),
            Kind::Expense => self.amount.negate(),
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        let wanted = normalise_tag(tag);
        self.tags.contains(&wanted)
    }
}
/// Everything needed to record a transaction; the ledger assigns the id.
#[derive(Debug, Clone, PartialEq)]
pub struct NewTransaction {
    pub date: Date,
    pub kind: Kind,
    pub amount: Money,
    pub category: Category,
    pub note: String,
    pub tags: Vec<String>,
}
