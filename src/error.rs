use std::fmt;
use std::io;

/// Every way a ledger operation can fail.
#[derive(Debug)]
pub enum Error {
    /// Input that could not be parsed: an amount, a date, a CSV row.
    Parse(String),
    /// An operation that would mix two currencies.
    CurrencyMismatch { left: String, right: String },
    /// A currency code the ledger does not know.
    UnknownCurrency(String),
    /// Arithmetic that would overflow the amount range.
    Overflow,
    /// A transaction, budget or rule that does not exist.
    NotFound(String),
    /// Bad command-line usage.
    Usage(String),
    /// A file could not be read or written.
    Io(io::Error),
}

/// Result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;
