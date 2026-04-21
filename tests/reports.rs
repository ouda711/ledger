use ledger::category::Category;
use ledger::date::Date;
use ledger::money::Money;
use ledger::query::{select, Filter};
use ledger::store::Ledger;
use ledger::txn::{parse_tags, Kind, NewTransaction};

#[allow(clippy::too_many_arguments)]
fn add(
    ledger: &mut Ledger,
    date: &str,
    kind: Kind,
    amount: &str,
    currency: &str,
    category: &str,
    note: &str,
    tags: &str,
) {
    ledger
        .add(NewTransaction {
            date: Date::parse(date).unwrap(),
            kind,
            amount: Money::parse(amount, currency).unwrap(),
            category: Category::new(category).unwrap(),
            note: note.into(),
            tags: parse_tags(tags),
        })
        .unwrap();
}

fn fixture() -> Ledger {
    let mut l = Ledger::new();
    add(
        &mut l,
        "2024-01-01",
        Kind::Income,
        "3000",
        "USD",
        "salary",
        "January pay",
        "",
    );
    add(
        &mut l,
        "2024-01-03",
        Kind::Expense,
        "120.40",
        "USD",
        "food/groceries",
        "Weekly shop",
        "",
    );
    add(
        &mut l,
        "2024-01-15",
        Kind::Expense,
        "35",
        "USD",
        "food/eating-out",
        "Pizza",
        "friends",
    );
    add(
        &mut l,
        "2024-01-20",
        Kind::Expense,
        "900",
        "USD",
        "rent",
        "",
        "",
    );
    add(
        &mut l,
        "2024-02-01",
        Kind::Income,
        "3000",
        "USD",
        "salary",
        "February pay",
        "",
    );
    add(
        &mut l,
        "2024-02-04",
        Kind::Expense,
        "80.10",
        "USD",
        "food/groceries",
        "",
        "",
    );
    add(
        &mut l,
        "2024-02-10",
        Kind::Expense,
        "15000",
        "KES",
        "transport",
        "Matatu fares",
        "work",
    );
    l
}

#[test]
fn empty_filter_selects_everything() {
    let l = fixture();
    assert!(Filter::default().is_empty());
    assert_eq!(select(l.transactions(), &Filter::default()).len(), 7);
}
