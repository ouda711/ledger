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
