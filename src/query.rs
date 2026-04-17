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
