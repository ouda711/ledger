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
#[test]
fn filters_by_month_and_range() {
    use ledger::date::Month;
    let l = fixture();
    let feb = Filter::for_month(Month::parse("2024-02").unwrap());
    assert_eq!(select(l.transactions(), &feb).len(), 3);
    let range = Filter {
        from: Some(Date::parse("2024-01-10").unwrap()),
        to: Some(Date::parse("2024-02-01").unwrap()),
        ..Filter::default()
    };
    assert_eq!(select(l.transactions(), &range).len(), 3);
}
#[test]
fn filters_by_attributes() {
    let l = fixture();
    let count = |f: Filter| select(l.transactions(), &f).len();
    assert_eq!(
        count(Filter {
            category: Some(Category::new("food").unwrap()),
            ..Default::default()
        }),
        3
    );
    assert_eq!(
        count(Filter {
            kind: Some(Kind::Income),
            ..Default::default()
        }),
        2
    );
    assert_eq!(
        count(Filter {
            tag: Some("#Friends".into()),
            ..Default::default()
        }),
        1
    );
    assert_eq!(
        count(Filter {
            text: Some("PAY".into()),
            ..Default::default()
        }),
        2
    );
    assert_eq!(
        count(Filter {
            text: Some("groc".into()),
            ..Default::default()
        }),
        2
    );
    assert_eq!(
        count(Filter {
            currency: Some("kes".into()),
            ..Default::default()
        }),
        1
    );
}
use ledger::report;

#[test]
fn totals_never_blend_currencies() {
    let l = fixture();
    let rows: Vec<_> = l.transactions().iter().collect();
    let totals = report::totals(&rows).unwrap();
    assert_eq!(totals.len(), 2);
    assert_eq!(totals[0].currency, "KES");
    assert_eq!(totals[0].expense.to_decimal(), "15000.00");
    assert_eq!(totals[1].currency, "USD");
    assert_eq!(totals[1].income.to_decimal(), "6000.00");
    assert_eq!(totals[1].expense.to_decimal(), "1135.50");
    assert_eq!(totals[1].net().unwrap().to_decimal(), "4864.50");
}
#[test]
fn category_breakdown_sorted_largest_first() {
    let l = fixture();
    let rows: Vec<_> = l.transactions().iter().collect();
    let groups = report::by_category(&rows, Kind::Expense, None).unwrap();
    let names: Vec<String> = groups.iter().map(|g| g.category.to_string()).collect();
    assert_eq!(
        names,
        ["transport", "rent", "food/groceries", "food/eating-out"]
    );
    assert_eq!(groups[2].count, 2);
    assert_eq!(groups[2].total.to_decimal(), "200.50");
}
#[test]
fn category_breakdown_rolls_up_by_depth() {
    let l = fixture();
    let rows: Vec<_> = l.transactions().iter().collect();
    let groups = report::by_category(&rows, Kind::Expense, Some(1)).unwrap();
    let food = groups
        .iter()
        .find(|g| g.category.as_str() == "food")
        .unwrap();
    assert_eq!(food.total.to_decimal(), "235.50");
    assert_eq!(food.count, 3);
}
