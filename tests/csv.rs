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
#[test]
fn sign_decides_kind_without_kind_column() {
    let items = csv::import("date,amount\n2024-03-01,-5\n2024-03-02,100\n", Some("KES")).unwrap();
    assert_eq!(items[0].kind, Kind::Expense);
    assert_eq!(items[0].amount.minor(), 500);
    assert_eq!(items[1].kind, Kind::Income);
    assert_eq!(items[0].category.as_str(), "uncategorised");
}
#[test]
fn missing_currency_is_refused_not_guessed() {
    let err = csv::import("date,amount\n2024-03-01,5\n", None).unwrap_err();
    assert!(err.to_string().contains("row 2"), "{err}");
}
#[test]
fn import_errors_name_the_row() {
    let text = "date,amount,currency\n2024-03-01,5,KES\n2024-03-02,5.555,KES\n";
    let err = csv::import(text, None).unwrap_err().to_string();
    assert!(err.starts_with("row 3:"), "{err}");
    assert!(csv::import("amount\n5\n", Some("KES")).is_err());
    assert!(csv::import("", Some("KES")).is_err());
}
#[test]
fn export_then_import_round_trips() {
    use ledger::store::Ledger;
    let items = csv::import(
        "date,amount,currency,category,note,tags\n2024-01-05,-3.20,EUR,coffee,\"flat white, large\",daily\n2024-01-06,1500,EUR,salary,,\n",
        None,
    )
    .unwrap();
    let mut ledger = Ledger::new();
    ledger.add_all(items.clone()).unwrap();
    let exported = csv::export(ledger.transactions());
    assert_eq!(csv::import(&exported, None).unwrap(), items);
}
