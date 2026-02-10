use crate::error::{parse_error, Error, Result};
use std::fmt;

/// A calendar date with no time zone, ordered chronologically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}
/// Gregorian leap year rule.
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Days in `month` (1-12) of `year`; 0 for an invalid month.
pub fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}
impl Date {
    /// A date, refusing ones that do not exist such as 2023-02-29.
    pub fn new(year: i32, month: u8, day: u8) -> Result<Date> {
        if !(1..=9999).contains(&year) {
            return Err(parse_error(format!("year {year} is out of range")));
        }
        if !(1..=12).contains(&month) {
            return Err(parse_error(format!("month {month} does not exist")));
        }
        if day == 0 || day > days_in_month(year, month) {
            return Err(parse_error(format!(
                "{year:04}-{month:02} has no day {day}"
            )));
        }
        Ok(Date { year, month, day })
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }
}
impl Date {
    /// Parse `YYYY-MM-DD`.
    pub fn parse(text: &str) -> Result<Date> {
        let t = text.trim();
        let bad = || parse_error(format!("expected a date like 2024-03-31, got '{t}'"));
        let parts: Vec<&str> = t.split('-').collect();
        if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
            return Err(bad());
        }
        if !parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit())) {
            return Err(bad());
        }
        let year = parts[0].parse().map_err(|_| bad())?;
        let month = parts[1].parse().map_err(|_| bad())?;
        let day = parts[2].parse().map_err(|_| bad())?;
        Date::new(year, month, day)
    }
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
impl Date {
    /// Days since 1970-01-01 (negative before it).
    /// Uses Howard Hinnant's civil-from-days algorithm.
    pub fn to_days(&self) -> i64 {
        let m = self.month as i64;
        let y = self.year as i64 - if m <= 2 { 1 } else { 0 };
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let mp = if m > 2 { m - 3 } else { m + 9 };
        let doy = (153 * mp + 2) / 5 + self.day as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    /// Inverse of [`Date::to_days`].
    pub fn from_days(days: i64) -> Result<Date> {
        let z = days.checked_add(719_468).ok_or(Error::Overflow)?;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 };
        let year = i32::try_from(year).map_err(|_| Error::Overflow)?;
        Date::new(year, month as u8, day as u8)
    }
}
