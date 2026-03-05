use ledger::category::Category;

fn c(name: &str) -> Category {
    Category::new(name).expect("valid category")
}

#[test]
fn categories_are_trimmed_and_lowercased() {
    assert_eq!(c(" Food / Groceries ").as_str(), "food/groceries");
    assert_eq!(Category::uncategorised().as_str(), "uncategorised");
}
#[test]
fn empty_segments_are_invalid() {
    for bad in ["", "/", "food/", "/food", "food//fruit", "  "] {
        assert!(Category::new(bad).is_err(), "accepted {bad:?}");
    }
}
