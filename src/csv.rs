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
