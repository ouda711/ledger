use crate::error::{parse_error, Error, Result};
use std::fmt;

/// A calendar date with no time zone, ordered chronologically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}
