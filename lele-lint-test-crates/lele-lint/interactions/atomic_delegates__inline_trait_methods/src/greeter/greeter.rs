use crate::greeter::traits::Greet;

pub struct Greeter {
    pub prefix: String,
    pub suffix: String,
}

impl Greet for Greeter {
    fn greet(&self, name: &str) -> String {
        let first = self.prefix.clone();
        let second = name.to_string();
        let third = self.suffix.clone();
        let mut out = String::new();
        out.push_str(&first);
        out.push_str(&second);
        out.push_str(&third);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::Greeter;
    use crate::greeter::traits::Greet;

    #[test]
    fn test_usage() {
        let greeter = Greeter {
            prefix: "hi ".to_string(),
            suffix: "!".to_string(),
        };
        assert_eq!(greeter.greet("al"), "hi al!");
    }
}
