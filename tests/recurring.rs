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
