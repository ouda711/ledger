use ledger::category::Category;
use ledger::date::Date;
use ledger::money::Money;
use ledger::txn::{parse_tags, Kind, NewTransaction};

fn new_txn(amount: &str, kind: Kind) -> NewTransaction {
    NewTransaction {
        date: Date::parse("2024-03-01").unwrap(),
        kind,
        amount: Money::parse(amount, "USD").unwrap(),
        category: Category::new("food").unwrap(),
        note: "lunch".into(),
        tags: vec![],
    }
}

#[test]
fn kinds_parse_from_common_words() {
    assert_eq!(Kind::parse("Income").unwrap(), Kind::Income);
    assert_eq!(Kind::parse("out").unwrap(), Kind::Expense);
    assert!(Kind::parse("transfer").is_err());
    assert_eq!(Kind::Expense.as_str(), "expense");
}
#[test]
fn signed_amount_follows_kind() {
    let expense = new_txn("5", Kind::Expense).into_transaction(1);
    let income = new_txn("5", Kind::Income).into_transaction(2);
    assert_eq!(expense.signed().unwrap().minor(), -500);
    assert_eq!(income.signed().unwrap().minor(), 500);
}
#[test]
fn tags_are_normalised_sorted_and_unique() {
    assert_eq!(parse_tags("#Trip, work,trip,, "), vec!["trip", "work"]);
    assert!(parse_tags("").is_empty());
    let mut t = new_txn("1", Kind::Expense).into_transaction(1);
    t.tags = parse_tags("trip");
    assert!(t.has_tag("#TRIP"));
    assert!(!t.has_tag("work"));
}
