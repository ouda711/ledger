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
fn width(text: &str) -> usize {
    text.chars().count()
}

fn pad(text: &str, to: usize, align: Align) -> String {
    let fill = " ".repeat(to.saturating_sub(width(text)));
    match align {
        Align::Left => format!("{text}{fill}"),
        Align::Right => format!("{fill}{text}"),
    }
}
impl Table {
    pub fn new(columns: &[(&str, Align)]) -> Table {
        Table {
            headers: columns.iter().map(|(h, _)| h.to_string()).collect(),
            aligns: columns.iter().map(|(_, a)| *a).collect(),
            rows: Vec::new(),
        }
    }

    /// Add a row; missing cells are blank and extra cells are dropped.
    pub fn push(&mut self, mut row: Vec<String>) {
        row.resize(self.headers.len(), String::new());
        self.rows.push(row);
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
