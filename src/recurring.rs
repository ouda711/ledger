use crate::date::Date;
use crate::error::{parse_error, Result};
use std::fmt;

/// How often a recurring transaction repeats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Every {
    Days(u32),
    Weeks(u32),
    Months(u32),
}
impl Every {
    /// `daily`, `weekly`, `monthly`, `yearly`, or a count and unit: `3d`, `2w`, `6m`.
    pub fn parse(text: &str) -> Result<Every> {
        let t = text.trim().to_ascii_lowercase();
        let named = match t.as_str() {
            "daily" => Some(Every::Days(1)),
            "weekly" => Some(Every::Weeks(1)),
            "fortnightly" => Some(Every::Weeks(2)),
            "monthly" => Some(Every::Months(1)),
            "quarterly" => Some(Every::Months(3)),
            "yearly" | "annually" => Some(Every::Months(12)),
            _ => None,
        };
        if let Some(every) = named {
            return Ok(every);
        }
        let bad = || parse_error(format!("'{text}' is not an interval; try monthly or 2w"));
        let unit = t.chars().last().ok_or_else(bad)?;
        let count: u32 = t[..t.len() - unit.len_utf8()].parse().map_err(|_| bad())?;
        if count == 0 {
            return Err(bad());
        }
        match unit {
            'd' => Ok(Every::Days(count)),
            'w' => Ok(Every::Weeks(count)),
            'm' => Ok(Every::Months(count)),
            'y' => Ok(Every::Months(count.checked_mul(12).ok_or_else(bad)?)),
            _ => Err(bad()),
        }
    }
}
impl fmt::Display for Every {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Every::Days(n) => write!(f, "{n}d"),
            Every::Weeks(n) => write!(f, "{n}w"),
            Every::Months(n) => write!(f, "{n}m"),
        }
    }
}
impl Every {
    /// The `n`th occurrence after `start` (the 0th is `start`).
    ///
    /// Always counted from the start date, so a rule starting on Jan 31 lands on
    /// Feb 28 and then Mar 31, instead of drifting to the 28th forever.
    pub fn nth(&self, start: Date, n: u32) -> Result<Date> {
        match *self {
            Every::Days(d) => start.add_days(d as i64 * n as i64),
            Every::Weeks(w) => start.add_days(7 * w as i64 * n as i64),
            Every::Months(m) => {
                let months = i32::try_from(m as i64 * n as i64)
                    .map_err(|_| crate::error::Error::Overflow)?;
                start.add_months(months)
            }
        }
    }
}
use crate::category::Category;
use crate::money::Money;
use crate::txn::{Kind, NewTransaction};

/// A transaction that repeats, such as rent or a salary.
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub id: u64,
    pub every: Every,
    pub start: Date,
    /// How many occurrences have already been written to the ledger.
    pub posted: u32,
    pub kind: Kind,
    pub amount: Money,
    pub category: Category,
    pub note: String,
}
impl Rule {
    /// The date of the next occurrence not yet posted.
    pub fn next_due(&self) -> Result<Date> {
        self.every.nth(self.start, self.posted)
    }

    /// Every occurrence due on or before `until`, marking them posted.
    pub fn take_due(&mut self, until: Date) -> Result<Vec<NewTransaction>> {
        const LIMIT: usize = 1000;
        let mut out = Vec::new();
        loop {
            let due = self.next_due()?;
            if due > until {
                break;
            }
            if out.len() == LIMIT {
                return Err(parse_error(format!(
                    "rule #{} has more than {LIMIT} occurrences due; check its start date",
                    self.id
                )));
            }
            out.push(NewTransaction {
                date: due,
                kind: self.kind,
                amount: self.amount,
                category: self.category.clone(),
                note: self.note.clone(),
                tags: vec!["recurring".to_string()],
            });
            self.posted += 1;
        }
        Ok(out)
    }
}
