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
impl Money {
    /// Build an amount from a count of minor units, e.g. `from_minor(1050, "USD")` is $10.50.
    pub fn from_minor(minor: i64, currency: &str) -> Result<Money> {
        let code = normalise_currency(currency)?;
        let bytes = code.as_bytes();
        Ok(Money {
            minor,
            currency: [bytes[0], bytes[1], bytes[2]],
        })
    }

    /// Zero in the given currency.
    pub fn zero(currency: &str) -> Result<Money> {
        Money::from_minor(0, currency)
    }

    /// The amount in minor units.
    pub fn minor(&self) -> i64 {
        self.minor
    }

    /// The ISO 4217 code.
    pub fn currency(&self) -> &str {
        std::str::from_utf8(&self.currency).expect("currency codes are ascii")
    }

    fn exponent(&self) -> u32 {
        currency_exponent(self.currency()).expect("currency validated on construction")
    }

    pub fn is_negative(&self) -> bool {
        self.minor < 0
    }

    pub fn is_zero(&self) -> bool {
        self.minor == 0
    }
}
impl Money {
    /// Parse text such as `"1,234.50"`, `"-12"` or `".5"` into minor units.
    ///
    /// More decimals than the currency has is refused rather than rounded: `10.005 KES`
    /// is a typo, and silently resolving it is how a cent goes missing.
    pub fn parse(text: &str, currency: &str) -> Result<Money> {
        let code = normalise_currency(currency)?;
        let exp = currency_exponent(&code)?;
        let cleaned: String = text
            .trim()
            .chars()
            .filter(|c| *c != ',' && *c != '_')
            .collect();
        let (negative, body) = match cleaned.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, cleaned.strip_prefix('+').unwrap_or(cleaned.as_str())),
        };
        let invalid = || parse_error(format!("'{}' is not an amount", text.trim()));
        if body.is_empty() {
            return Err(invalid());
        }
        let (whole, frac) = match body.split_once('.') {
            Some((_, "")) => return Err(invalid()),
            Some((w, f)) => (w, f),
            None => (body, ""),
        };
        let digits_only = |s: &str| s.chars().all(|c| c.is_ascii_digit());
        if !digits_only(whole) || !digits_only(frac) {
            return Err(invalid());
        }
        if frac.len() as u32 > exp {
            return Err(parse_error(format!(
                "'{}' has more than {exp} decimal place(s), which {code} does not allow",
                text.trim()
            )));
        }
        let mut digits = String::from(if whole.is_empty() { "0" } else { whole });
        digits.push_str(frac);
        for _ in frac.len() as u32..exp {
            digits.push('0');
        }
        let value: i64 = digits.parse().map_err(|_| Error::Overflow)?;
        Money::from_minor(if negative { -value } else { value }, &code)
    }
}
impl Money {
    /// The amount as a plain decimal with exactly the currency's places, e.g. `"-1234.50"`.
    pub fn to_decimal(&self) -> String {
        let exp = self.exponent() as usize;
        let sign = if self.minor < 0 { "-" } else { "" };
        let digits = self.minor.unsigned_abs().to_string();
        if exp == 0 {
            return format!("{sign}{digits}");
        }
        let padded = format!("{:0>width$}", digits, width = exp + 1);
        let (whole, frac) = padded.split_at(padded.len() - exp);
        format!("{sign}{whole}.{frac}")
    }
}
fn group_thousands(digits: &str) -> String {
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let decimal = self.to_decimal();
        let (sign, rest) = match decimal.strip_prefix('-') {
            Some(r) => ("-", r),
            None => ("", decimal.as_str()),
        };
        let (whole, frac) = match rest.split_once('.') {
            Some((w, fr)) => (w, Some(fr)),
            None => (rest, None),
        };
        write!(f, "{} {sign}{}", self.currency(), group_thousands(whole))?;
        if let Some(fr) = frac {
            write!(f, ".{fr}")?;
        }
        Ok(())
    }
}
impl Money {
    fn same_currency(&self, other: &Money) -> Result<()> {
        if self.currency == other.currency {
            Ok(())
        } else {
            Err(Error::CurrencyMismatch {
                left: self.currency().to_string(),
                right: other.currency().to_string(),
            })
        }
    }

    pub fn checked_add(&self, other: &Money) -> Result<Money> {
        self.same_currency(other)?;
        let minor = self.minor.checked_add(other.minor).ok_or(Error::Overflow)?;
        Ok(Money { minor, ..*self })
    }

    pub fn checked_sub(&self, other: &Money) -> Result<Money> {
        self.same_currency(other)?;
        let minor = self.minor.checked_sub(other.minor).ok_or(Error::Overflow)?;
        Ok(Money { minor, ..*self })
    }

    pub fn negate(&self) -> Result<Money> {
        let minor = self.minor.checked_neg().ok_or(Error::Overflow)?;
        Ok(Money { minor, ..*self })
    }

    pub fn abs(&self) -> Result<Money> {
        let minor = self.minor.checked_abs().ok_or(Error::Overflow)?;
        Ok(Money { minor, ..*self })
    }
}
impl Money {
    /// Order two amounts; comparing different currencies is an error, not `false`.
    pub fn compare(&self, other: &Money) -> Result<Ordering> {
        self.same_currency(other)?;
        Ok(self.minor.cmp(&other.minor))
    }
}
/// Total of `items`, all of which must be in `currency`.
pub fn sum<'a, I>(currency: &str, items: I) -> Result<Money>
where
    I: IntoIterator<Item = &'a Money>,
{
    let mut total = Money::zero(currency)?;
    for m in items {
        total = total.checked_add(m)?;
    }
    Ok(total)
}
