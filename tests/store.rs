use ledger::category::Category;
use ledger::date::Date;
use ledger::money::Money;
use ledger::store::Ledger;
use ledger::txn::{Kind, NewTransaction};

fn item(date: &str, amount: &str, note: &str) -> NewTransaction {
    NewTransaction {
        date: Date::parse(date).unwrap(),
        kind: Kind::Expense,
        amount: Money::parse(amount, "KES").unwrap(),
        category: Category::new("food").unwrap(),
        note: note.into(),
        tags: vec![],
    }
}

#[test]
fn add_assigns_increasing_ids() {
    let mut ledger = Ledger::new();
    assert!(ledger.is_empty());
    assert_eq!(ledger.add(item("2024-01-02", "10", "a")).unwrap(), 1);
    assert_eq!(ledger.add(item("2024-01-01", "20", "b")).unwrap(), 2);
    assert_eq!(ledger.len(), 2);
}
#[test]
fn transactions_are_sorted_by_date_then_id() {
    let mut ledger = Ledger::new();
    ledger.add(item("2024-01-03", "1", "c")).unwrap();
    ledger.add(item("2024-01-01", "1", "a")).unwrap();
    ledger.add(item("2024-01-03", "1", "d")).unwrap();
    let notes: Vec<&str> = ledger
        .transactions()
        .iter()
        .map(|t| t.note.as_str())
        .collect();
    assert_eq!(notes, ["a", "c", "d"]);
}
