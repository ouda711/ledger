use crate::error::{Error, Result};
use crate::txn::{NewTransaction, Transaction};

/// All recorded transactions, kept sorted by date then id.
#[derive(Debug, Clone, PartialEq)]
pub struct Ledger {
    transactions: Vec<Transaction>,
    next_id: u64,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger {
            transactions: Vec::new(),
            next_id: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger::new()
    }
}
impl Ledger {
    /// Record a transaction and return its new id. Ids are never reused.
    pub fn add(&mut self, new: NewTransaction) -> Result<u64> {
        new.validate()?;
        let id = self.next_id;
        self.next_id += 1;
        self.transactions.push(new.into_transaction(id));
        self.sort();
        Ok(id)
    }

    fn sort(&mut self) {
        self.transactions.sort_by_key(|t| (t.date, t.id));
    }
}
impl Ledger {
    pub fn get(&self, id: u64) -> Option<&Transaction> {
        self.transactions.iter().find(|t| t.id == id)
    }

    pub fn remove(&mut self, id: u64) -> Result<Transaction> {
        let index = self
            .transactions
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| Error::NotFound(format!("transaction #{id}")))?;
        Ok(self.transactions.remove(index))
    }
}
impl Ledger {
    /// Apply `change` to a copy of the transaction and keep it only if still valid.
    pub fn update<F>(&mut self, id: u64, change: F) -> Result<()>
    where
        F: FnOnce(&mut Transaction),
    {
        let slot = self
            .transactions
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| Error::NotFound(format!("transaction #{id}")))?;
        let mut edited = slot.clone();
        change(&mut edited);
        edited.id = id;
        crate::txn::validate_amount(&edited.amount)?;
        crate::txn::validate_note(&edited.note)?;
        *slot = edited;
        self.sort();
        Ok(())
    }
}
/// Escape backslashes, tabs and newlines so a field fits on one TSV line.
pub(crate) fn escape(field: &str) -> String {
    let mut out = String::with_capacity(field.len());
    for c in field.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            other => out.push(other),
        }
    }
    out
}

/// Reverse of [`escape`].
pub(crate) fn unescape(field: &str) -> String {
    let mut out = String::with_capacity(field.len());
    let mut chars = field.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}
const HEADER_PREFIX: &str = "# ledger v1";
fn to_line(t: &Transaction) -> String {
    [
        t.id.to_string(),
        t.date.to_string(),
        t.kind.as_str().to_string(),
        t.amount.to_decimal(),
        t.amount.currency().to_string(),
        escape(t.category.as_str()),
        t.tags.join(","),
        escape(&t.note),
    ]
    .join("\t")
}
