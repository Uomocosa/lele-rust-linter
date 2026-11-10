use crate::Linter;

pub fn print_rule_list(linter: &Linter) {
    let width = linter
        .rules
        .iter()
        .map(|rule| rule.code().len())
        .max()
        .unwrap_or_default();
    for rule in &linter.rules {
        println!("{:>width$}  {}", rule.code(), rule.name());
    }
}

#[cfg(test)]
mod tests {
    use super::print_rule_list;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let linter = Linter::default();
        print_rule_list(&linter);
        assert!(!linter.rules.is_empty());
    }
}
