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
