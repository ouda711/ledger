/// Column alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Right,
}

/// A table rendered with aligned columns and a rule under the header.
#[derive(Debug, Clone, Default)]
pub struct Table {
    headers: Vec<String>,
    aligns: Vec<Align>,
    rows: Vec<Vec<String>>,
}
