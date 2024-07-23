use std::env::current_dir;

#[test]
fn hoge() {
    assert_eq!(current_dir().ok(), None)
}
