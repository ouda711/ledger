use ledger::csv;

#[test]
fn parses_simple_records() {
    let records = csv::parse("a,b,c\n1,2,3\n").unwrap();
    assert_eq!(records, vec![vec!["a", "b", "c"], vec!["1", "2", "3"]]);
}
#[test]
fn handles_quotes_commas_and_newlines() {
    let records = csv::parse("\"a,b\",\"say \"\"hi\"\"\",\"two\nlines\"\r\nx,,z").unwrap();
    assert_eq!(records[0], vec!["a,b", "say \"hi\"", "two\nlines"]);
    assert_eq!(records[1], vec!["x", "", "z"]);
}
