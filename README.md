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

## Recording transactions

```sh
export LEDGER_CURRENCY=KES

ledger add 450 --category food/lunch --note "Chapati and beans"
ledger add 85000 --income --category salary --date 2024-03-01
ledger add 1200 --category transport --tags work,client-visit --date yesterday
```

Transactions are expenses unless you pass `--income`. Categories nest with `/`.

## Listing, editing and removing

```sh
ledger list                         # everything
ledger list --month 2024-03         # one month
ledger list --category food         # food and everything under it
ledger list --tag work --limit 10   # the last ten work expenses
ledger show 12
ledger edit 12 --amount 500 --note "Lunch with team"
ledger rm 12
```

`ledger list --json` prints the same transactions as a JSON array for scripts. Amounts are
signed decimal strings (`"-12.50"`), never JSON numbers, so no reader can turn them into floats.

## Reports

```sh
ledger summary --month 2024-03      # income, expense, net per currency
ledger months                       # month-by-month totals
ledger categories --depth 1         # where the money went, rolled up
ledger summary --json               # the same totals as JSON, one object per currency
```

```
Category   Count          Total
---------  -----  -------------
rent           1  KES 35,000.00
food           9   KES 8,450.00
transport      6   KES 4,200.00
```

## Budgets

```sh
ledger budget set food 10000
ledger budget set transport 6000
ledger budget                       # this month
ledger budget --month 2024-02
```

A budget covers its category and everything nested under it.

## Recurring transactions

```sh
ledger recurring add 35000 --every monthly --start 2024-01-01 --category rent
ledger recurring add 85000 --income --every monthly --start 2024-01-25 --category salary
ledger recurring run                # post everything due up to today
```

Intervals: `daily`, `weekly`, `fortnightly`, `monthly`, `quarterly`, `yearly`, or a count and
unit such as `3d`, `2w`, `6m`. A monthly rule that starts on the 31st falls on the last day of
shorter months and returns to the 31st afterwards. It does not drift.

## Import and export

```sh
ledger import statement.csv --currency KES --dry-run
ledger import statement.csv --currency KES
ledger export march.csv --month 2024-03
```

Import needs `date` and `amount` columns. Optional columns: `kind`, `currency`, `category`,
`tags`, `note` (or `description`/`memo`). Without a `kind` column, negative amounts are
expenses. An import is all or nothing: one bad row and nothing is added.

## Storage

Data lives in three tab-separated files in `LEDGER_DIR`: `ledger.tsv`, `budgets.tsv` and
`recurring.tsv`. They are plain text, safe to keep in git, and written atomically so a crash
never leaves half a file. See [docs/FILE_FORMAT.md](docs/FILE_FORMAT.md).

## Development

```sh
cargo test
cargo clippy --all-targets
cargo fmt --check
```

## Licence

MIT. See [LICENSE](LICENSE).
