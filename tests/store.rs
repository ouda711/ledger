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
#[test]
fn render_and_parse_round_trip() {
    let mut ledger = Ledger::new();
    let mut tricky = item(
        "2024-02-29",
        "1234.56",
        "tab\there, slash \\ and \\n literal",
    );
    tricky.tags = vec!["trip".into(), "work".into()];
    ledger.add(tricky).unwrap();
    ledger.add(item("2024-03-01", "0.01", "")).unwrap();
    let text = ledger.render_text();
    assert_eq!(text.lines().count(), 3);
    assert_eq!(Ledger::parse_text(&text).unwrap(), ledger);
}
#[test]
fn next_id_is_persisted() {
    let mut ledger = Ledger::new();
    let id = ledger.add(item("2024-01-01", "1", "a")).unwrap();
    ledger.remove(id).unwrap();
    let mut reloaded = Ledger::parse_text(&ledger.render_text()).unwrap();
    assert_eq!(reloaded.add(item("2024-01-01", "1", "b")).unwrap(), id + 1);
}
#[test]
fn parse_errors_name_the_line() {
    let text = "# ledger v1 next=3\n1\t2024-01-01\texpense\t1.00\tKES\tfood\t\t\n2\tnot-a-date\texpense\t1.00\tKES\tfood\t\t\n";
    let err = Ledger::parse_text(text).unwrap_err().to_string();
    assert!(err.starts_with("line 3:"), "{err}");
}
#[test]
fn duplicate_ids_are_rejected() {
    let line = "1\t2024-01-01\texpense\t1.00\tKES\tfood\t\t";
    let text = format!("{line}\n{line}\n");
    assert!(Ledger::parse_text(&text).is_err());
}
#[test]
fn save_and_load_from_disk() {
    let dir = std::env::temp_dir().join(format!("ledger-store-test-{}", std::process::id()));
    let path = dir.join("nested").join("ledger.tsv");
    assert!(Ledger::load(&path).unwrap().is_empty());
    let mut ledger = Ledger::new();
    ledger.add(item("2024-01-01", "9.99", "saved")).unwrap();
    ledger.save(&path).unwrap();
    assert_eq!(Ledger::load(&path).unwrap(), ledger);
    assert!(!path.with_extension("tmp").exists());
    std::fs::remove_dir_all(dir).unwrap();
}
#[test]
fn add_all_adds_nothing_when_one_item_is_bad() {
    let mut ledger = Ledger::new();
    let bad = item("2024-01-02", "0", "zero");
    let err = ledger
        .add_all(vec![item("2024-01-01", "1", "ok"), bad])
        .unwrap_err();
    assert!(err.to_string().contains("item 2"));
    assert!(ledger.is_empty());
    assert_eq!(
        ledger.add_all(vec![item("2024-01-01", "1", "ok")]).unwrap(),
        vec![1]
    );
}
