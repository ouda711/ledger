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
