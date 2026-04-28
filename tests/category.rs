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
#[test]
fn hierarchy() {
    let fruit = c("food/groceries/fruit");
    assert_eq!(fruit.parent(), Some(c("food/groceries")));
    assert_eq!(fruit.top(), c("food"));
    assert_eq!(fruit.depth(), 3);
    assert_eq!(c("food").parent(), None);
}
#[test]
fn is_within_respects_segment_boundaries() {
    assert!(c("food/groceries").is_within(&c("food")));
    assert!(c("food").is_within(&c("food")));
    assert!(!c("foodstuff").is_within(&c("food")));
    assert!(!c("food").is_within(&c("food/groceries")));
}
#[test]
fn truncate_to_depth() {
    let fruit = c("food/groceries/fruit");
    assert_eq!(fruit.truncate(1), c("food"));
    assert_eq!(fruit.truncate(2), c("food/groceries"));
    assert_eq!(fruit.truncate(9), fruit);
    assert_eq!(fruit.truncate(0), c("food"));
}
