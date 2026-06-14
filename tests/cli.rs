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
