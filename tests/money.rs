use ledger::error::Error;
use ledger::money::{self, Money};

#[test]
fn from_minor_keeps_units_and_uppercases_currency() {
    let money = Money::from_minor(1050, "usd").unwrap();
    assert_eq!(money.minor(), 1050);
    assert_eq!(money.currency(), "USD");
}
#[test]
fn unknown_currency_is_an_error_not_a_default() {
    assert!(matches!(
        Money::from_minor(1, "XYZ"),
        Err(Error::UnknownCurrency(code)) if code == "XYZ"
    ));
}
fn m(text: &str, currency: &str) -> Money {
    Money::parse(text, currency).expect("valid amount")
}

#[test]
fn parses_whole_and_fractional_amounts() {
    assert_eq!(m("12", "KES").minor(), 1200);
    assert_eq!(m("12.5", "KES").minor(), 1250);
    assert_eq!(m("12.05", "KES").minor(), 1205);
    assert_eq!(m(".5", "KES").minor(), 50);
}
#[test]
fn parses_signs_and_thousand_separators() {
    assert_eq!(m("-3.20", "USD").minor(), -320);
    assert_eq!(m("+3", "USD").minor(), 300);
    assert_eq!(m("1,234,567.89", "USD").minor(), 123_456_789);
    assert_eq!(m("1_000", "USD").minor(), 100_000);
}
#[test]
fn currencies_without_minor_units() {
    assert_eq!(m("5000", "UGX").minor(), 5000);
    assert!(Money::parse("5000.5", "UGX").is_err());
}
#[test]
fn three_decimal_currencies() {
    assert_eq!(m("1.234", "KWD").minor(), 1234);
    assert_eq!(m("1.2", "KWD").minor(), 1200);
}
#[test]
fn excess_precision_is_refused_not_rounded() {
    let err = Money::parse("10.005", "KES").unwrap_err();
    assert!(err.to_string().contains("decimal place"));
}
#[test]
fn rejects_malformed_amounts() {
    for bad in ["", "-", "abc", "1.2.3", "1.", "1e5", "--1", "12 34"] {
        assert!(Money::parse(bad, "USD").is_err(), "accepted {bad:?}");
    }
}
