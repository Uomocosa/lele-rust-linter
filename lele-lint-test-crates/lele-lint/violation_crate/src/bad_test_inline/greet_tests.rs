#[test]
fn extra_case() {
    assert_eq!(crate::bad_test_inline::load_data(), vec![1, 2, 3]);
}
