use crate::error::{Error, Result};
use crate::txn::{NewTransaction, Transaction};

/// All recorded transactions, kept sorted by date then id.
#[derive(Debug, Clone, PartialEq)]
pub struct Ledger {
    transactions: Vec<Transaction>,
    next_id: u64,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger {
            transactions: Vec::new(),
            next_id: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
}

impl Default for Ledger {
    fn default() -> Self {
        Ledger::new()
    }
}
impl Ledger {
    /// Record a transaction and return its new id. Ids are never reused.
    pub fn add(&mut self, new: NewTransaction) -> Result<u64> {
        new.validate()?;
        let id = self.next_id;
        self.next_id += 1;
        self.transactions.push(new.into_transaction(id));
        self.sort();
        Ok(id)
    }

    fn sort(&mut self) {
        self.transactions.sort_by_key(|t| (t.date, t.id));
    }
}
impl Ledger {
    pub fn get(&self, id: u64) -> Option<&Transaction> {
        self.transactions.iter().find(|t| t.id == id)
    }

    pub fn remove(&mut self, id: u64) -> Result<Transaction> {
        let index = self
            .transactions
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| Error::NotFound(format!("transaction #{id}")))?;
        Ok(self.transactions.remove(index))
    }
}
impl Ledger {
    /// Apply `change` to a copy of the transaction and keep it only if still valid.
    pub fn update<F>(&mut self, id: u64, change: F) -> Result<()>
    where
        F: FnOnce(&mut Transaction),
    {
        let slot = self
            .transactions
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| Error::NotFound(format!("transaction #{id}")))?;
        let mut edited = slot.clone();
        change(&mut edited);
        edited.id = id;
        crate::txn::validate_amount(&edited.amount)?;
        crate::txn::validate_note(&edited.note)?;
        *slot = edited;
        self.sort();
        Ok(())
    }
}
/// Escape backslashes, tabs and newlines so a field fits on one TSV line.
pub(crate) fn escape(field: &str) -> String {
    let mut out = String::with_capacity(field.len());
    for c in field.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            other => out.push(other),
        }
    }
    out
}

/// Reverse of [`escape`].
pub(crate) fn unescape(field: &str) -> String {
    let mut out = String::with_capacity(field.len());
    let mut chars = field.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}
const HEADER_PREFIX: &str = "# ledger v1";
fn to_line(t: &Transaction) -> String {
    [
        t.id.to_string(),
        t.date.to_string(),
        t.kind.as_str().to_string(),
        t.amount.to_decimal(),
        t.amount.currency().to_string(),
        escape(t.category.as_str()),
        t.tags.join(","),
        escape(&t.note),
    ]
    .join("\t")
}
fn from_line(line: &str, line_no: usize) -> Result<Transaction> {
    use crate::category::Category;
    use crate::date::Date;
    use crate::money::Money;
    use crate::txn::{parse_tags, Kind};

    let at = |e: Error| match e {
        Error::Parse(msg) => Error::Parse(format!("line {line_no}: {msg}")),
        other => other,
    };
    let fields: Vec<&str> = line.split('\t').collect();
    if fields.len() != 8 {
        return Err(at(Error::Parse(format!(
            "expected 8 fields, found {}",
            fields.len()
        ))));
    }
    let id = fields[0]
        .parse::<u64>()
        .map_err(|_| at(Error::Parse(format!("bad id '{}'", fields[0]))))?;
    Ok(Transaction {
        id,
        date: Date::parse(fields[1]).map_err(at)?,
        kind: Kind::parse(fields[2]).map_err(at)?,
        amount: Money::parse(fields[3], fields[4]).map_err(at)?,
        category: Category::new(&unescape(fields[5])).map_err(at)?,
        tags: parse_tags(fields[6]),
        note: unescape(fields[7]),
    })
}
impl Ledger {
    /// The on-disk form: a header carrying the next id, then one line per transaction.
    pub fn render_text(&self) -> String {
        let mut out = format!("{HEADER_PREFIX} next={}\n", self.next_id);
        for t in &self.transactions {
            out.push_str(&to_line(t));
            out.push('\n');
        }
        out
    }
}
impl Ledger {
    pub fn parse_text(text: &str) -> Result<Ledger> {
        let mut ledger = Ledger::new();
        let mut declared_next = 1;
        for (index, line) in text.lines().enumerate() {
            let line_no = index + 1;
            if line.trim().is_empty() {
                continue;
            }
            if let Some(rest) = line.strip_prefix(HEADER_PREFIX) {
                if let Some(n) = rest.trim().strip_prefix("next=") {
                    declared_next = n
                        .parse()
                        .map_err(|_| Error::Parse(format!("line {line_no}: bad next id '{n}'")))?;
                }
                continue;
            }
            if line.starts_with('#') {
                continue;
            }
            let t = from_line(line, line_no)?;
            if ledger.get(t.id).is_some() {
                return Err(Error::Parse(format!(
                    "line {line_no}: duplicate transaction id {}",
                    t.id
                )));
            }
            ledger.transactions.push(t);
        }
        let max_id = ledger.transactions.iter().map(|t| t.id).max().unwrap_or(0);
        ledger.next_id = declared_next.max(max_id + 1);
        ledger.sort();
        Ok(ledger)
    }
}
use std::fs;
use std::io;
use std::path::Path;

/// The file's contents, or `None` if it does not exist yet.
pub(crate) fn read_optional(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}
/// Write via a temporary file and rename, so a crash never leaves half a ledger.
pub(crate) fn write_atomic(path: &Path, contents: &str) -> Result<()> {
    if let Some(dir) = path.parent() {
        if !dir.as_os_str().is_empty() {
            fs::create_dir_all(dir)?;
        }
    }
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, contents)?;
    fs::rename(&tmp, path)?;
    Ok(())
}
