use crate::error::{parse_error, Error, Result};
use std::cmp::Ordering;
use std::fmt;

/// Currencies the ledger understands, with the number of digits in their minor unit.
///
/// Not every currency has two decimals: UGX and XOF have none, KWD has three.
const CURRENCIES: &[(&str, u32)] = &[
    ("AED", 2),
    ("BHD", 3),
    ("BWP", 2),
    ("CAD", 2),
    ("CHF", 2),
    ("EGP", 2),
    ("ETB", 2),
    ("EUR", 2),
    ("GBP", 2),
    ("GHS", 2),
    ("JPY", 0),
    ("KES", 2),
    ("KWD", 3),
    ("MAD", 2),
    ("NGN", 2),
    ("OMR", 3),
    ("RWF", 0),
    ("TND", 3),
    ("TZS", 2),
    ("UGX", 0),
    ("USD", 2),
    ("XAF", 0),
    ("XOF", 0),
    ("ZAR", 2),
    ("ZMW", 2),
];
/// Number of decimal places for `code`. Unknown codes are an error, never a default:
/// guessing two decimals for a currency that has none is a 100x mistake.
pub fn currency_exponent(code: &str) -> Result<u32> {
    CURRENCIES
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, e)| *e)
        .ok_or_else(|| Error::UnknownCurrency(code.to_string()))
}
/// Every supported currency code, alphabetically.
pub fn supported_currencies() -> impl Iterator<Item = &'static str> {
    CURRENCIES.iter().map(|(c, _)| *c)
}

/// Upper-case and validate a currency code.
pub fn normalise_currency(code: &str) -> Result<String> {
    let upper = code.trim().to_ascii_uppercase();
    currency_exponent(&upper)?;
    Ok(upper)
}
/// An amount of money as a whole count of the currency's minor unit.
///
/// Floating point cannot represent decimal money exactly, so it is never used here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Money {
    minor: i64,
    currency: [u8; 3],
}
