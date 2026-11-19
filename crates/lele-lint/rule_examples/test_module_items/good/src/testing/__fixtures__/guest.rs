use crate::stock;

pub fn guest() -> stock::Guest {
    stock::Guest("Ada".to_string())
}

#[cfg(test)]
mod tests {
    use super::guest;

    #[test]
    fn test_usage() {
        assert_eq!(guest().greet(), "Hello, Ada!");
    }
}
