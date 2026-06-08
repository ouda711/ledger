use crate::error::{Error, Result};
use crate::money::normalise_currency;
use std::path::PathBuf;

/// Where data lives and which currency to assume.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub dir: PathBuf,
    /// Used when a command gives no `--currency`. There is deliberately no built-in default.
    pub currency: Option<String>,
}
