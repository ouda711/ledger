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
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(msg) => write!(f, "{msg}"),
            Error::CurrencyMismatch { left, right } => {
                write!(
                    f,
                    "cannot combine {left} with {right}; amounts in different currencies never mix"
                )
            }
            Error::UnknownCurrency(code) => write!(f, "unknown currency '{code}'"),
            Error::Overflow => write!(f, "amount is too large"),
            Error::NotFound(what) => write!(f, "{what} not found"),
            Error::Usage(msg) => write!(f, "{msg}"),
            Error::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
/// Shorthand for building an [`Error::Parse`].
pub fn parse_error(msg: impl Into<String>) -> Error {
    Error::Parse(msg.into())
}
