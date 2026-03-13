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
