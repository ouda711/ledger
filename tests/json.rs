use ledger::category::Category;
use ledger::date::Date;
use ledger::json;
use ledger::money::Money;
use ledger::txn::{Kind, NewTransaction};

#[test]
fn strings_escape_quotes_backslashes_and_control_characters() {
    assert_eq!(json::string("plain"), "\"plain\"");
    assert_eq!(json::string("say \"hi\""), "\"say \\\"hi\\\"\"");
    assert_eq!(json::string("a\\b\tc\nd"), "\"a\\\\b\\tc\\nd\"");
    assert_eq!(json::string("\u{1}"), "\"\\u0001\"");
    assert_eq!(json::string("héllo ✓"), "\"héllo ✓\"");
}

#[test]
fn transactions_use_signed_decimal_strings() {
    let t = NewTransaction {
        date: Date::parse("2024-03-01").unwrap(),
        kind: Kind::Expense,
        amount: Money::parse("0.10", "USD").unwrap(),
        category: Category::new("food/coffee").unwrap(),
        note: "flat \"white\"".into(),
        tags: vec!["daily".into(), "work".into()],
    }
    .into_transaction(7);
    assert_eq!(
        json::transaction(&t).unwrap(),
        r#"{"id":7,"date":"2024-03-01","kind":"expense","amount":"-0.10","currency":"USD","category":"food/coffee","tags":["daily","work"],"note":"flat \"white\""}"#
    );
}

#[test]
fn arrays_put_one_object_per_line() {
    assert_eq!(json::array(&[]), "[]\n");
    assert_eq!(
        json::array(&["{\"a\":1}".into(), "{\"b\":2}".into()]),
        "[\n  {\"a\":1},\n  {\"b\":2}\n]\n"
    );
}
