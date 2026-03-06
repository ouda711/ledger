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
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl Category {
    /// `food/groceries` -> `food`; `None` for a top-level category.
    pub fn parent(&self) -> Option<Category> {
        self.0
            .rsplit_once('/')
            .map(|(parent, _)| Category(parent.to_string()))
    }

    /// The first segment: `food/groceries/fruit` -> `food`.
    pub fn top(&self) -> Category {
        Category(self.0.split('/').next().unwrap_or_default().to_string())
    }

    /// True when `self` is `other` or nested anywhere beneath it.
    pub fn is_within(&self, other: &Category) -> bool {
        self.0 == other.0
            || (self.0.starts_with(&other.0) && self.0[other.0.len()..].starts_with('/'))
    }

    /// Nesting depth: `food` is 1, `food/groceries` is 2.
    pub fn depth(&self) -> usize {
        self.0.split('/').count()
    }
}
