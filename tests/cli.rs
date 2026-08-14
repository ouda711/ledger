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
