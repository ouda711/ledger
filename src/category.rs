use crate::error::{parse_error, Result};
use std::fmt;

/// A spending or income category. Stored lowercase; `/` nests, as in `food/groceries`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category(String);
