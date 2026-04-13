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
use crate::category::Category;
use crate::date::Date;
use crate::money::Money;
use crate::txn::{parse_tags, Kind, NewTransaction};

/// Read transactions from CSV with a header row.
///
/// `date` and `amount` columns are required. Without a `kind` column a negative amount
/// is an expense. Without a `currency` column, `default_currency` is used; if that is
/// missing too the import is refused rather than guessed.
pub fn import(text: &str, default_currency: Option<&str>) -> Result<Vec<NewTransaction>> {
    let mut records = parse(text)?.into_iter();
    let header = records
        .next()
        .ok_or_else(|| parse_error("the CSV file is empty"))?;
    let date_col = column(&header, &["date"]).ok_or_else(|| parse_error("no 'date' column"))?;
    let amount_col =
        column(&header, &["amount", "value"]).ok_or_else(|| parse_error("no 'amount' column"))?;
    let kind_col = column(&header, &["kind", "type"]);
    let currency_col = column(&header, &["currency"]);
    let category_col = column(&header, &["category"]);
    let tags_col = column(&header, &["tags"]);
    let note_col = column(&header, &["note", "description", "memo"]);

    let mut out = Vec::new();
    for (index, record) in records.enumerate() {
        let row = index + 2;
        let cell = |col: Option<usize>| col.and_then(|c| record.get(c)).map(|s| s.trim());
        let fail = |msg: String| parse_error(format!("row {row}: {msg}"));
        let currency = match cell(currency_col).filter(|s| !s.is_empty()) {
            Some(c) => c.to_string(),
            None => default_currency
                .ok_or_else(|| {
                    fail("no currency; add a currency column or pass --currency".into())
                })?
                .to_string(),
        };
        let raw_amount = cell(Some(amount_col)).unwrap_or("");
        let amount = Money::parse(raw_amount, &currency).map_err(|e| fail(e.to_string()))?;
        let kind = match cell(kind_col).filter(|s| !s.is_empty()) {
            Some(k) => Kind::parse(k).map_err(|e| fail(e.to_string()))?,
            None if amount.is_negative() => Kind::Expense,
            None => Kind::Income,
        };
        let category = match cell(category_col).filter(|s| !s.is_empty()) {
            Some(c) => Category::new(c).map_err(|e| fail(e.to_string()))?,
            None => Category::uncategorised(),
        };
        let item = NewTransaction {
            date: Date::parse(cell(Some(date_col)).unwrap_or(""))
                .map_err(|e| fail(e.to_string()))?,
            kind,
            amount: amount.abs()?,
            category,
            note: cell(note_col).unwrap_or("").replace(['\n', '\r'], " "),
            tags: parse_tags(cell(tags_col).unwrap_or("")),
        };
        item.validate().map_err(|e| fail(e.to_string()))?;
        out.push(item);
    }
    Ok(out)
}
