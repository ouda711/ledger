use crate::error::{parse_error, Result};

/// Parse CSV text into records, honouring quoted fields, doubled quotes (`""`)
/// and line breaks inside quotes. Blank lines are skipped.
pub fn parse(text: &str) -> Result<Vec<Vec<String>>> {
    let mut records = Vec::new();
    let mut record = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();
    let mut line = 1;

    while let Some(c) = chars.next() {
        if in_quotes {
            match c {
                '"' if chars.peek() == Some(&'"') => {
                    chars.next();
                    field.push('"');
                }
                '"' => in_quotes = false,
                '\n' => {
                    line += 1;
                    field.push(c);
                }
                _ => field.push(c),
            }
            continue;
        }
        match c {
            '"' if field.is_empty() => in_quotes = true,
            ',' => record.push(std::mem::take(&mut field)),
            '\r' => {}
            '\n' => {
                line += 1;
                record.push(std::mem::take(&mut field));
                if !(record.len() == 1 && record[0].is_empty()) {
                    records.push(std::mem::take(&mut record));
                } else {
                    record.clear();
                }
            }
            _ => field.push(c),
        }
    }
    if in_quotes {
        return Err(parse_error(format!("unclosed quote before line {line}")));
    }
    if !field.is_empty() || !record.is_empty() {
        record.push(field);
        records.push(record);
    }
    Ok(records)
}
/// Quote a field if it contains a comma, quote or line break.
pub fn quote(field: &str) -> String {
    if field.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

/// One CSV line, without the trailing newline.
pub fn write_record<S: AsRef<str>>(fields: &[S]) -> String {
    fields
        .iter()
        .map(|f| quote(f.as_ref()))
        .collect::<Vec<_>>()
        .join(",")
}
use crate::txn::Transaction;

/// Columns written by [`export`] and understood by [`import`].
pub const COLUMNS: [&str; 7] = [
    "date", "kind", "amount", "currency", "category", "tags", "note",
];

pub fn export<'a, I>(transactions: I) -> String
where
    I: IntoIterator<Item = &'a Transaction>,
{
    let mut out = write_record(&COLUMNS);
    out.push('\n');
    for t in transactions {
        let row = [
            t.date.to_string(),
            t.kind.as_str().to_string(),
            t.amount.to_decimal(),
            t.amount.currency().to_string(),
            t.category.to_string(),
            t.tags.join(","),
            t.note.clone(),
        ];
        out.push_str(&write_record(&row));
        out.push('\n');
    }
    out
}
fn column(header: &[String], names: &[&str]) -> Option<usize> {
    header.iter().position(|h| {
        let h = h.trim().to_ascii_lowercase();
        names.iter().any(|n| *n == h)
    })
}
