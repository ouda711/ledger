use crate::error::{Error, Result};
use std::collections::{BTreeMap, BTreeSet};

/// Flags that take no value.
const SWITCHES: &[&str] = &["income", "expense", "help", "dry-run", "version"];

/// Parsed arguments: a command, positional words, `--flag value` pairs and `--switch`es.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Args {
    pub command: Option<String>,
    pub positional: Vec<String>,
    flags: BTreeMap<String, String>,
    switches: BTreeSet<String>,
}
