use ledger::table::{truncate, Align, Table};

#[test]
fn renders_aligned_columns() {
    let mut t = Table::new(&[("Name", Align::Left), ("Amount", Align::Right)]);
    t.push(vec!["rent".into(), "900.00".into()]);
    t.push(vec!["coffee".into(), "3.20".into()]);
    let expected = "\
Name    Amount
------  ------
rent    900.00
coffee    3.20
";
    assert_eq!(t.render(), expected);
}
#[test]
fn pads_short_rows_and_drops_extra_cells() {
    let mut t = Table::new(&[("A", Align::Left), ("B", Align::Left)]);
    assert!(t.is_empty());
    t.push(vec!["only".into()]);
    t.push(vec!["x".into(), "y".into(), "ignored".into()]);
    let out = t.render();
    assert!(!out.contains("ignored"));
    assert_eq!(out.lines().nth(2), Some("only"));
}
#[test]
fn truncates_with_ellipsis() {
    assert_eq!(truncate("short", 10), "short");
    assert_eq!(truncate("a long note", 6), "a lon…");
    assert_eq!(truncate("héllo wörld", 5), "héll…");
    assert_eq!(truncate("x", 0), "");
}
