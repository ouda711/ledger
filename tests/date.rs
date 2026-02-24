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
#[test]
fn day_numbers_round_trip_over_four_centuries() {
    let start = d("1900-01-01").to_days();
    for n in (start..start + 146_097).step_by(97) {
        assert_eq!(Date::from_days(n).unwrap().to_days(), n);
    }
}
#[test]
fn add_days_crosses_months_and_years() {
    assert_eq!(d("2024-02-28").add_days(1).unwrap(), d("2024-02-29"));
    assert_eq!(d("2023-02-28").add_days(1).unwrap(), d("2023-03-01"));
    assert_eq!(d("2024-12-31").add_days(1).unwrap(), d("2025-01-01"));
    assert_eq!(d("2025-01-01").add_days(-1).unwrap(), d("2024-12-31"));
    assert_eq!(d("2024-01-01").days_until(&d("2025-01-01")), 366);
}
#[test]
fn weekdays() {
    assert_eq!(d("1970-01-01").weekday(), 3);
    assert_eq!(d("2024-03-04").weekday(), 0);
    assert_eq!(d("2024-03-10").weekday(), 6);
}
#[test]
fn add_months_clamps_to_month_end() {
    assert_eq!(d("2024-01-31").add_months(1).unwrap(), d("2024-02-29"));
    assert_eq!(d("2023-01-31").add_months(1).unwrap(), d("2023-02-28"));
    assert_eq!(d("2024-03-31").add_months(-1).unwrap(), d("2024-02-29"));
    assert_eq!(d("2024-11-15").add_months(3).unwrap(), d("2025-02-15"));
    assert_eq!(d("2024-05-20").last_of_month(), d("2024-05-31"));
    assert_eq!(d("2024-05-20").first_of_month(), d("2024-05-01"));
}
#[test]
fn months_parse_and_bound_dates() {
    use ledger::date::Month;
    let month = Month::parse("2024-02").unwrap();
    assert_eq!(month.to_string(), "2024-02");
    assert_eq!(month.first_day(), d("2024-02-01"));
    assert_eq!(month.last_day(), d("2024-02-29"));
    assert!(month.contains(&d("2024-02-15")));
    assert!(!month.contains(&d("2024-03-01")));
    assert!(Month::parse("2024-13").is_err());
    assert!(Month::parse("2024").is_err());
}
