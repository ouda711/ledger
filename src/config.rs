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
impl Config {
    /// `LEDGER_DIR` (else `$HOME/.ledger`) and `LEDGER_CURRENCY`.
    pub fn from_env() -> Result<Config> {
        let dir = match std::env::var_os("LEDGER_DIR") {
            Some(d) if !d.is_empty() => PathBuf::from(d),
            _ => {
                let home = std::env::var_os("HOME")
                    .or_else(|| std::env::var_os("USERPROFILE"))
                    .ok_or_else(|| {
                        Error::Usage("set LEDGER_DIR to choose where data is kept".into())
                    })?;
                PathBuf::from(home).join(".ledger")
            }
        };
        let currency = match std::env::var("LEDGER_CURRENCY") {
            Ok(c) if !c.trim().is_empty() => Some(normalise_currency(&c)?),
            _ => None,
        };
        Ok(Config { dir, currency })
    }
}
