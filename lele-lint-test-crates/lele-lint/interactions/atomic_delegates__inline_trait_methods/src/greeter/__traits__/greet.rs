pub trait Greet {
    fn greet(&self, name: &str) -> String;
}

#[rustfmt::skip]
impl Greet for String {
    fn greet(&self, name: &str) -> String { format!("{self}{name}") }
}

#[cfg(test)]
mod tests {
    use super::Greet;

    #[test]
    fn test_usage() {
        assert_eq!("hi ".to_string().greet("al"), "hi al");
    }
}
