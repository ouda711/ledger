use ledger::csv;

#[test]
fn parses_simple_records() {
    let records = csv::parse("a,b,c\n1,2,3\n").unwrap();
    assert_eq!(records, vec![vec!["a", "b", "c"], vec!["1", "2", "3"]]);
}
