use ledger::date::{days_in_month, is_leap_year, Date};

fn d(text: &str) -> Date {
    Date::parse(text).expect("valid date")
}

#[test]
fn leap_year_rule() {
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2000));
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(days_in_month(2023, 2), 28);
    assert_eq!(days_in_month(2023, 13), 0);
}
#[test]
fn impossible_dates_are_refused() {
    assert!(Date::new(2023, 2, 29).is_err());
    assert!(Date::new(2024, 4, 31).is_err());
    assert!(Date::new(2024, 0, 1).is_err());
    assert!(Date::new(0, 1, 1).is_err());
    assert!(Date::new(2024, 2, 29).is_ok());
}
#[test]
fn parses_iso_dates() {
    let date = d("2024-03-09");
    assert_eq!((date.year(), date.month(), date.day()), (2024, 3, 9));
    for bad in [
        "2024-3-9",
        "24-03-09",
        "2024/03/09",
        "2024-03-09x",
        "",
        "abcd-ef-gh",
    ] {
        assert!(Date::parse(bad).is_err(), "accepted {bad:?}");
    }
}
#[test]
fn displays_in_iso_format() {
    assert_eq!(Date::new(987, 1, 2).unwrap().to_string(), "0987-01-02");
    assert_eq!(d("2024-12-31").to_string(), "2024-12-31");
}
#[test]
fn day_numbers_known_values() {
    assert_eq!(d("1970-01-01").to_days(), 0);
    assert_eq!(d("1970-01-02").to_days(), 1);
    assert_eq!(d("1969-12-31").to_days(), -1);
    assert_eq!(d("2000-03-01").to_days(), 11_017);
}
