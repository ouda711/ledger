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
