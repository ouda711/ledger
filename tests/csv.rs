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
#[test]
fn unclosed_quote_is_an_error_and_blank_lines_skip() {
    assert!(csv::parse("\"open,1\n").is_err());
    assert_eq!(csv::parse("a\n\n\nb\n").unwrap().len(), 2);
}
#[test]
fn quotes_only_when_needed() {
    assert_eq!(csv::quote("plain"), "plain");
    assert_eq!(csv::quote("a,b"), "\"a,b\"");
    assert_eq!(csv::quote("say \"hi\""), "\"say \"\"hi\"\"\"");
    assert_eq!(csv::write_record(&["a", "b,c"]), "a,\"b,c\"");
}
use ledger::txn::Kind;

#[test]
fn imports_all_columns() {
    let text = "Date,Kind,Amount,Currency,Category,Tags,Note\n2024-03-01,expense,12.50,usd,Food/Lunch,\"work,team\",Sandwich\n";
    let items = csv::import(text, None).unwrap();
    assert_eq!(items.len(), 1);
    let t = &items[0];
    assert_eq!(t.kind, Kind::Expense);
    assert_eq!(t.amount.to_string(), "USD 12.50");
    assert_eq!(t.category.as_str(), "food/lunch");
    assert_eq!(t.tags, vec!["team", "work"]);
    assert_eq!(t.note, "Sandwich");
}
