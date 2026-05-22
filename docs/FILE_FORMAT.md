# File formats

All files are UTF-8, tab-separated, one record per line. Lines starting with `#` are comments,
except the headers described below. Fields escape backslash, tab, newline and carriage return
as `\\`, `\t`, `\n` and `\r`.

## ledger.tsv

The first line is a header carrying the next id to assign, so ids are never reused even after
the newest transaction is deleted:

```
# ledger v1 next=42
```

Each transaction has eight fields:

| # | Field    | Example          | Notes                                        |
|---|----------|------------------|----------------------------------------------|
| 1 | id       | `17`             | Unique, positive                             |
| 2 | date     | `2024-03-01`     | ISO 8601                                     |
| 3 | kind     | `expense`        | `income` or `expense`                        |
| 4 | amount   | `1250.00`        | Always positive, exactly the currency's decimals |
| 5 | currency | `KES`            | ISO 4217                                     |
| 6 | category | `food/groceries` | Lowercase, `/` nests                         |
| 7 | tags     | `trip,work`      | Comma-separated, may be empty                |
| 8 | note     | `Weekly shop`    | Escaped, may be empty                        |

## budgets.tsv

Three fields per line: category, monthly limit, currency.

```
food	10000.00	KES
rent	35000.00	KES
```
