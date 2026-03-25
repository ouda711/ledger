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
#[test]
fn remove_returns_the_transaction() {
    let mut ledger = Ledger::new();
    let id = ledger.add(item("2024-01-01", "1", "a")).unwrap();
    assert_eq!(ledger.remove(id).unwrap().note, "a");
    assert!(ledger.remove(id).is_err());
    assert!(ledger.get(id).is_none());
}
#[test]
fn ids_are_not_reused_after_removal() {
    let mut ledger = Ledger::new();
    let first = ledger.add(item("2024-01-01", "1", "a")).unwrap();
    ledger.remove(first).unwrap();
    assert_eq!(ledger.add(item("2024-01-01", "1", "b")).unwrap(), first + 1);
}
#[test]
fn update_rejects_invalid_edits() {
    let mut ledger = Ledger::new();
    let id = ledger.add(item("2024-01-01", "5", "a")).unwrap();
    ledger.update(id, |t| t.note = "changed".into()).unwrap();
    assert_eq!(ledger.get(id).unwrap().note, "changed");
    let zero = Money::zero("KES").unwrap();
    assert!(ledger.update(id, |t| t.amount = zero).is_err());
    assert_eq!(ledger.get(id).unwrap().amount.minor(), 500);
    assert!(ledger.update(99, |_| {}).is_err());
}
