use ledger::config::Config;
use std::path::PathBuf;

#[test]
fn data_files_live_in_the_data_dir() {
    let config = Config {
        dir: PathBuf::from("/data"),
        currency: None,
    };
    let dir = PathBuf::from("/data");
    assert_eq!(config.ledger_path(), dir.join("ledger.tsv"));
    assert_eq!(config.budgets_path(), dir.join("budgets.tsv"));
    assert_eq!(config.rules_path(), dir.join("recurring.tsv"));
}
