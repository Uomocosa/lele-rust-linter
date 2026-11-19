pub struct Guest(pub String);

#[rustfmt::skip]
impl Guest {
    pub fn greet(&self) -> String { format!("Hello, {}!", **self) }
}

#[cfg(test)]
mod tests {
    use super::Guest;

    fn make_guest() -> Guest {
        Guest("Ada".to_string())
    }

    #[test]
    fn test_usage() {
        assert_eq!(make_guest().greet(), "Hello, Ada!");
    }
}
