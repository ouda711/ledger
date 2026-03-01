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
impl Date {
    pub fn add_days(&self, n: i64) -> Result<Date> {
        Date::from_days(self.to_days().checked_add(n).ok_or(Error::Overflow)?)
    }

    /// Whole days from `self` to `other`; negative when `other` is earlier.
    pub fn days_until(&self, other: &Date) -> i64 {
        other.to_days() - self.to_days()
    }

    /// Monday is 0, Sunday is 6. 1970-01-01 was a Thursday.
    pub fn weekday(&self) -> u8 {
        (self.to_days() + 3).rem_euclid(7) as u8
    }
}
impl Date {
    /// `n` months later (or earlier). Jan 31 plus one month is Feb 28/29, not Mar 3.
    pub fn add_months(&self, n: i32) -> Result<Date> {
        let total = self
            .year
            .checked_mul(12)
            .and_then(|t| t.checked_add(self.month as i32 - 1))
            .and_then(|t| t.checked_add(n))
            .ok_or(Error::Overflow)?;
        let year = total.div_euclid(12);
        let month = (total.rem_euclid(12) + 1) as u8;
        let day = self.day.min(days_in_month(year, month));
        Date::new(year, month, day)
    }

    pub fn first_of_month(&self) -> Date {
        Date { day: 1, ..*self }
    }

    pub fn last_of_month(&self) -> Date {
        Date {
            day: days_in_month(self.year, self.month),
            ..*self
        }
    }
}
/// A calendar month, used to group and filter transactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Month {
    pub year: i32,
    pub month: u8,
}

impl Month {
    pub fn of(date: &Date) -> Month {
        Month {
            year: date.year,
            month: date.month,
        }
    }

    /// Parse `YYYY-MM`.
    pub fn parse(text: &str) -> Result<Month> {
        let t = text.trim();
        let bad = || parse_error(format!("expected a month like 2024-03, got '{t}'"));
        let (y, m) = t.split_once('-').ok_or_else(bad)?;
        if y.len() != 4 || m.len() != 2 {
            return Err(bad());
        }
        let year: i32 = y.parse().map_err(|_| bad())?;
        let month: u8 = m.parse().map_err(|_| bad())?;
        Date::new(year, month, 1)?;
        Ok(Month { year, month })
    }

    pub fn first_day(&self) -> Date {
        Date {
            year: self.year,
            month: self.month,
            day: 1,
        }
    }

    pub fn last_day(&self) -> Date {
        self.first_day().last_of_month()
    }

    pub fn contains(&self, date: &Date) -> bool {
        date.year == self.year && date.month == self.month
    }
}
impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.month)
    }
}
impl Month {
    pub fn next(&self) -> Result<Month> {
        Ok(Month::of(&self.first_day().add_months(1)?))
    }

    pub fn previous(&self) -> Result<Month> {
        Ok(Month::of(&self.first_day().add_months(-1)?))
    }
}
/// Today in UTC.
pub fn today() -> Date {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    Date::from_days(secs.div_euclid(86_400)).expect("system clock within year 9999")
}
/// Parse `today`, `yesterday`, `-3` (days ago) or an ISO date, relative to `now`.
pub fn parse_relative(text: &str, now: Date) -> Result<Date> {
    match text.trim().to_ascii_lowercase().as_str() {
        "today" => Ok(now),
        "yesterday" => now.add_days(-1),
        t if t.starts_with('-') && t[1..].chars().all(|c| c.is_ascii_digit()) && t.len() > 1 => {
            let n: i64 = t[1..].parse().map_err(|_| Error::Overflow)?;
            now.add_days(-n)
        }
        _ => Date::parse(text),
    }
}
