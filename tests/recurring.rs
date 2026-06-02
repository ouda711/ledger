use ledger::date::Date;
use ledger::recurring::Every;

fn d(text: &str) -> Date {
    Date::parse(text).unwrap()
}

#[test]
fn parses_named_and_counted_intervals() {
    assert_eq!(Every::parse("Monthly").unwrap(), Every::Months(1));
    assert_eq!(Every::parse("fortnightly").unwrap(), Every::Weeks(2));
    assert_eq!(Every::parse("yearly").unwrap(), Every::Months(12));
    assert_eq!(Every::parse("3d").unwrap(), Every::Days(3));
    assert_eq!(Every::parse("2y").unwrap(), Every::Months(24));
    for bad in ["", "0d", "m", "3x", "often"] {
        assert!(Every::parse(bad).is_err(), "accepted {bad:?}");
    }
    assert_eq!(Every::Weeks(2).to_string(), "2w");
}
#[test]
fn monthly_from_the_31st_does_not_drift() {
    let start = d("2024-01-31");
    let every = Every::Months(1);
    assert_eq!(every.nth(start, 1).unwrap(), d("2024-02-29"));
    assert_eq!(every.nth(start, 2).unwrap(), d("2024-03-31"));
    assert_eq!(every.nth(start, 3).unwrap(), d("2024-04-30"));
    assert_eq!(
        Every::Weeks(2).nth(d("2024-01-01"), 3).unwrap(),
        d("2024-02-12")
    );
}
use ledger::category::Category;
use ledger::money::Money;
use ledger::recurring::{Rule, Rules};
use ledger::txn::Kind;

fn rent() -> Rule {
    Rule {
        id: 0,
        every: Every::Months(1),
        start: d("2024-01-01"),
        posted: 0,
        kind: Kind::Expense,
        amount: Money::parse("900", "USD").unwrap(),
        category: Category::new("rent").unwrap(),
        note: "Flat".into(),
    }
}

#[test]
fn take_due_posts_each_occurrence_once() {
    let mut rule = rent();
    let due = rule.take_due(d("2024-03-15")).unwrap();
    let dates: Vec<String> = due.iter().map(|t| t.date.to_string()).collect();
    assert_eq!(dates, ["2024-01-01", "2024-02-01", "2024-03-01"]);
    assert_eq!(due[0].tags, vec!["recurring"]);
    assert_eq!(rule.next_due().unwrap(), d("2024-04-01"));
    assert!(rule.take_due(d("2024-03-31")).unwrap().is_empty());
}
#[test]
fn runaway_rules_are_refused() {
    let mut rule = rent();
    rule.every = Every::Days(1);
    rule.start = d("2000-01-01");
    assert!(rule.take_due(d("2024-01-01")).is_err());
}
#[test]
fn rules_assign_ids_and_merge_due_items_by_date() {
    let mut rules = Rules::default();
    assert_eq!(rules.add(rent()).unwrap(), 1);
    let mut salary = rent();
    salary.kind = Kind::Income;
    salary.start = d("2024-01-25");
    assert_eq!(rules.add(salary).unwrap(), 2);
    let due = rules.take_due(d("2024-02-28")).unwrap();
    let dates: Vec<String> = due.iter().map(|t| t.date.to_string()).collect();
    assert_eq!(
        dates,
        ["2024-01-01", "2024-01-25", "2024-02-01", "2024-02-25"]
    );
    assert!(rules.remove(1).is_ok());
    assert!(rules.remove(1).is_err());
}
