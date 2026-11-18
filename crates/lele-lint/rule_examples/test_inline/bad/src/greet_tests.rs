#[test]
fn greets_bob() {
    assert_eq!(crate::Guest("Bob".to_string()).greet(), "Hello, Bob!");
}
