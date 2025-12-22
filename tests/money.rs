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
