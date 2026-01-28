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
