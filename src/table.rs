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
