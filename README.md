# ledger

Track income and spending from the command line. Plain-text storage, exact money, no dependencies.

## Install

```sh
git clone <this repo>
cd ledger
cargo install --path .
```

Requires Rust 1.75 or newer. The crate has no dependencies outside the standard library.

## Money is exact

Amounts are stored as whole minor units (cents, kobo, fils) together with their ISO 4217
currency code, never as floating point. That means:

- `0.10 + 0.20` is exactly `0.30`.
- Currencies with no minor unit (UGX, XOF, RWF, JPY) and with three decimals (KWD, BHD, OMR, TND)
  are handled correctly.
- Typing more decimals than a currency has (`10.005` KES) is an error, not silently rounded.
- Amounts in different currencies are never added together. Reports show one line per currency.

## Dates

Anywhere a date is accepted you can write `2024-03-31`, `today`, `yesterday`, or `-3`
(three days ago).

## Configuration

| Variable          | Meaning                                                        |
| ----------------- | -------------------------------------------------------------- |
| `LEDGER_DIR`      | Where data is kept. Defaults to `~/.ledger`.                   |
| `LEDGER_CURRENCY` | Currency used when a command has no `--currency`. No default.  |

There is deliberately no built-in default currency: guessing wrong is a silent 100x error.
