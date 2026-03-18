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
