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
