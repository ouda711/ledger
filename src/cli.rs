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
impl Args {
    pub fn parse(raw: &[String]) -> Result<Args> {
        let mut args = Args::default();
        let mut words = Vec::new();
        let mut iter = raw.iter();
        let mut only_words = false;
        while let Some(arg) = iter.next() {
            if only_words {
                words.push(arg.clone());
            } else if arg == "--" {
                only_words = true;
            } else if arg == "-h" {
                args.switches.insert("help".into());
            } else if let Some(flag) = arg.strip_prefix("--") {
                match flag.split_once('=') {
                    Some((name, value)) => {
                        args.flags.insert(name.to_string(), value.to_string());
                    }
                    None if SWITCHES.contains(&flag) => {
                        args.switches.insert(flag.to_string());
                    }
                    None => {
                        let value = iter
                            .next()
                            .ok_or_else(|| Error::Usage(format!("--{flag} needs a value")))?;
                        args.flags.insert(flag.to_string(), value.clone());
                    }
                }
            } else {
                words.push(arg.clone());
            }
        }
        let mut words = words.into_iter();
        args.command = words.next();
        args.positional = words.collect();
        Ok(args)
    }
}
