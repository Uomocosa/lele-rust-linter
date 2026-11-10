pub trait Greetable {
    fn greet(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::Greetable;

    struct Demo;

    impl Greetable for Demo {
        fn greet(&self) -> String {
            "hello".to_string()
        }
    }

    #[test]
    fn test_usage() {
        assert_eq!(Demo.greet(), "hello");
    }
}
