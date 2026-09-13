# Contributing

Thanks for taking a look.

## Ground rules

- **No floating point for money.** Use `Money`; add a currency to the table in `src/money.rs`
  with its correct exponent if you need one.
- **Never default a currency.** If one is not given, refuse with a clear message.
- **Refuse, don't round.** Bad input is an error that names the line or row.
- **No dependencies** unless there is a very good reason. The standard library goes a long way.

## Before you open a pull request

```sh
cargo fmt
cargo clippy --all-targets
cargo test
```

Every change comes with tests, including the ways it should fail.
