//! `ledger` keeps a personal record of income and spending in plain text files.
//!
//! Money is stored as whole minor units (cents, kobo, fils) with its ISO 4217 code,
//! never as floating point, and amounts in different currencies are never added together.

pub mod error;
pub mod money;
pub mod date;
pub mod category;
pub mod txn;
pub mod store;
