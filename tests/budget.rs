use ledger::budget::{status, Budget, Budgets};
use ledger::category::Category;
use ledger::date::{Date, Month};
use ledger::money::Money;
use ledger::store::Ledger;
use ledger::txn::{Kind, NewTransaction};

fn spend(ledger: &mut Ledger, date: &str, amount: &str, currency: &str, category: &str) {
    ledger
        .add(NewTransaction {
            date: Date::parse(date).unwrap(),
            kind: Kind::Expense,
            amount: Money::parse(amount, currency).unwrap(),
            category: Category::new(category).unwrap(),
            note: String::new(),
            tags: vec![],
        })
        .unwrap();
}

fn budget(category: &str, limit: &str) -> Budget {
    Budget {
        category: Category::new(category).unwrap(),
        limit: Money::parse(limit, "USD").unwrap(),
    }
}

#[test]
fn status_counts_nested_spending_in_the_month() {
    let mut l = Ledger::new();
    spend(&mut l, "2024-03-02", "100", "USD", "food/groceries");
    spend(&mut l, "2024-03-09", "50", "USD", "food");
    spend(&mut l, "2024-04-01", "999", "USD", "food");
    spend(&mut l, "2024-03-10", "999", "KES", "food");
    spend(&mut l, "2024-03-10", "999", "USD", "rent");
    let s = status(
        &budget("food", "400"),
        l.transactions(),
        Month::parse("2024-03").unwrap(),
    )
    .unwrap();
    assert_eq!(s.spent.to_decimal(), "150.00");
    assert_eq!(s.remaining.to_decimal(), "250.00");
    assert_eq!(s.used_bp, Some(3750));
    assert!(!s.is_over());
}
#[test]
fn overspending_shows_negative_remaining() {
    let mut l = Ledger::new();
    spend(&mut l, "2024-03-02", "120", "USD", "fun");
    let s = status(
        &budget("fun", "100"),
        l.transactions(),
        Month::parse("2024-03").unwrap(),
    )
    .unwrap();
    assert!(s.is_over());
    assert_eq!(s.remaining.to_decimal(), "-20.00");
    assert_eq!(s.used_bp, Some(12_000));
}
#[test]
fn set_replaces_and_remove_deletes() {
    let mut b = Budgets::default();
    b.set(budget("food", "400")).unwrap();
    b.set(budget("food", "500")).unwrap();
    b.set(budget("rent", "900")).unwrap();
    assert_eq!(b.iter().count(), 2);
    assert_eq!(b.iter().next().unwrap().limit.to_decimal(), "500.00");
    b.remove(&Category::new("food").unwrap()).unwrap();
    assert!(b.remove(&Category::new("food").unwrap()).is_err());
    assert!(b.set(budget("x", "-1")).is_err());
}
#[test]
fn budgets_round_trip() {
    let mut b = Budgets::default();
    b.set(budget("food/groceries", "400.50")).unwrap();
    b.set(Budget {
        category: Category::new("rent").unwrap(),
        limit: Money::parse("45000", "KES").unwrap(),
    })
    .unwrap();
    assert_eq!(Budgets::parse_text(&b.render_text()).unwrap(), b);
    assert!(Budgets::parse_text("food\t1\n").is_err());
}
