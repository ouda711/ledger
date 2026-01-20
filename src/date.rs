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
