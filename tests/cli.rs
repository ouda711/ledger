use ledger::cli::Args;

fn strings(raw: &[&str]) -> Vec<String> {
    raw.iter().map(|s| s.to_string()).collect()
}

#[test]
fn parses_commands_flags_and_switches() {
    let args = Args::parse(&strings(&[
        "add",
        "12.50",
        "--category",
        "food",
        "--note=lunch",
        "--income",
    ]))
    .unwrap();
    assert_eq!(args.command.as_deref(), Some("add"));
    assert_eq!(args.positional(0), Some("12.50"));
    assert_eq!(args.value("category"), Some("food"));
    assert_eq!(args.value("note"), Some("lunch"));
    assert!(args.switch("income"));
    assert!(!args.switch("expense"));
}
#[test]
fn missing_values_and_double_dash() {
    assert!(Args::parse(&strings(&["add", "--category"])).is_err());
    let args = Args::parse(&strings(&["add", "--", "-5"])).unwrap();
    assert_eq!(args.positional(0), Some("-5"));
}
#[test]
fn unknown_flags_are_rejected() {
    let args = Args::parse(&strings(&["list", "--catgory", "food"])).unwrap();
    assert!(args.allow(&["category"]).is_err());
    assert!(args.allow(&["catgory"]).is_ok());
}
use ledger::cli::{run, Context};
use ledger::config::Config;
use ledger::date::Date;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

struct Env {
    config: Config,
    today: Date,
}

impl Env {
    fn new() -> Env {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir: PathBuf =
            std::env::temp_dir().join(format!("ledger-cli-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        Env {
            config: Config {
                dir,
                currency: Some("USD".into()),
            },
            today: Date::parse("2024-03-15").unwrap(),
        }
    }

    fn try_run(&self, raw: &[&str]) -> ledger::Result<String> {
        let ctx = Context {
            config: &self.config,
            today: self.today,
        };
        let mut out = Vec::new();
        run(&strings(raw), &ctx, &mut out)?;
        Ok(String::from_utf8(out).unwrap())
    }

    fn run(&self, raw: &[&str]) -> String {
        self.try_run(raw)
            .unwrap_or_else(|e| panic!("{raw:?} failed: {e}"))
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.config.dir);
    }
}
#[test]
fn add_records_an_expense_by_default() {
    let env = Env::new();
    let out = env.run(&["add", "12.50", "--category", "Food", "--note", "lunch"]);
    assert_eq!(out, "Added #1: expense USD 12.50 on 2024-03-15 (food)\n");
    let out = env.run(&[
        "add",
        "3000",
        "--income",
        "--date",
        "yesterday",
        "--currency",
        "kes",
    ]);
    assert!(out.contains("income KES 3,000.00 on 2024-03-14"), "{out}");
}
#[test]
fn add_without_any_currency_is_a_usage_error() {
    let mut env = Env::new();
    env.config.currency = None;
    let err = env.try_run(&["add", "5"]).unwrap_err();
    assert!(matches!(err, ledger::Error::Usage(_)));
    assert!(env
        .try_run(&["add", "5", "--income", "--expense", "--currency", "USD"])
        .is_err());
}
#[test]
fn list_shows_signed_amounts_and_filters() {
    let env = Env::new();
    assert_eq!(env.run(&["list"]), "No transactions.\n");
    env.run(&["add", "10", "--category", "food", "--date", "2024-03-01"]);
    env.run(&[
        "add",
        "2000",
        "--income",
        "--category",
        "salary",
        "--date",
        "2024-02-28",
    ]);
    let out = env.run(&["list"]);
    assert!(out.contains("USD -10.00"), "{out}");
    assert!(out.contains("USD 2,000.00"), "{out}");
    let food = env.run(&["list", "--category", "food"]);
    assert!(!food.contains("salary"));
    let march = env.run(&["list", "--month", "2024-03"]);
    assert_eq!(march.lines().count(), 3);
    let last = env.run(&["list", "--limit", "1"]);
    assert!(last.contains("food") && !last.contains("salary"));
}
#[test]
fn show_and_remove() {
    let env = Env::new();
    env.run(&["add", "7", "--tags", "trip", "--note", "taxi"]);
    let out = env.run(&["show", "#1"]);
    assert!(
        out.contains("Tags      trip") && out.contains("Note      taxi"),
        "{out}"
    );
    assert!(env.run(&["rm", "1"]).starts_with("Removed #1"));
    assert!(matches!(
        env.try_run(&["show", "1"]),
        Err(ledger::Error::NotFound(_))
    ));
    assert!(env.try_run(&["rm", "abc"]).is_err());
}
#[test]
fn edit_changes_only_given_fields() {
    let env = Env::new();
    env.run(&["add", "7", "--category", "taxi", "--note", "airport"]);
    env.run(&[
        "edit",
        "1",
        "--amount",
        "8.25",
        "--category",
        "transport/taxi",
    ]);
    let out = env.run(&["show", "1"]);
    assert!(
        out.contains("USD 8.25") && out.contains("transport/taxi") && out.contains("airport"),
        "{out}"
    );
    assert!(env.try_run(&["edit", "1"]).is_err());
    assert!(env.try_run(&["edit", "1", "--amount", "0"]).is_err());
    assert!(env.try_run(&["edit", "1", "--amount", "1.001"]).is_err());
}
#[test]
fn summary_per_currency() {
    let env = Env::new();
    env.run(&["add", "100", "--income"]);
    env.run(&["add", "30.25"]);
    env.run(&["add", "500", "--currency", "KES"]);
    let out = env.run(&["summary"]);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 4, "{out}");
    assert!(
        lines[2].starts_with("KES") && lines[2].ends_with("-500.00"),
        "{out}"
    );
    assert!(
        lines[3].starts_with("USD") && lines[3].ends_with("69.75"),
        "{out}"
    );
}
