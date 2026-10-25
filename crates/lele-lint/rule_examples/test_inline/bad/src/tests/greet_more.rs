#[test]
fn greets_empty_name() {
    assert_eq!(crate::Guest(String::new()).greet(), "Hello, !");
}
