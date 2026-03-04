use crate::error::{parse_error, Result};
use std::fmt;

/// A spending or income category. Stored lowercase; `/` nests, as in `food/groceries`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Category(String);
impl Category {
    pub fn new(name: &str) -> Result<Category> {
        let parts: Vec<String> = name.split('/').map(|p| p.trim().to_lowercase()).collect();
        if parts.iter().any(|p| p.is_empty()) {
            return Err(parse_error(format!("'{name}' is not a valid category")));
        }
        if parts.iter().any(|p| p.chars().any(|c| c.is_control())) {
            return Err(parse_error("categories cannot contain control characters"));
        }
        Ok(Category(parts.join("/")))
    }

    /// The category used when none is given.
    pub fn uncategorised() -> Category {
        Category("uncategorised".to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
