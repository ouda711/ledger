//! Just enough JSON writing for machine-readable command output.
//!
//! Amounts are written as decimal strings, not JSON numbers: many JSON readers parse numbers
//! as 64-bit floats, which would quietly turn `0.10` into `0.1000000000000000055…`.

use crate::error::Result;
use crate::txn::Transaction;
use std::fmt::Write;

/// `text` as a quoted JSON string, escaping quotes, backslashes and control characters.
pub fn string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// A JSON array of strings.
pub fn string_array(items: &[String]) -> String {
    let parts: Vec<String> = items.iter().map(|s| string(s)).collect();
    format!("[{}]", parts.join(","))
}

/// One transaction as a JSON object. `amount` is signed: negative for expenses.
pub fn transaction(t: &Transaction) -> Result<String> {
    Ok(format!(
        "{{\"id\":{},\"date\":{},\"kind\":{},\"amount\":{},\"currency\":{},\"category\":{},\"tags\":{},\"note\":{}}}",
        t.id,
        string(&t.date.to_string()),
        string(t.kind.as_str()),
        string(&t.signed()?.to_decimal()),
        string(t.amount.currency()),
        string(t.category.as_str()),
        string_array(&t.tags),
        string(&t.note),
    ))
}

/// Objects as a JSON array, one per line, so the output is easy to read and to diff.
pub fn array(objects: &[String]) -> String {
    if objects.is_empty() {
        return "[]\n".to_string();
    }
    format!("[\n  {}\n]\n", objects.join(",\n  "))
}
