use crate::Linter;

pub fn print_checker_list(linter: &Linter) {
    let width = linter
        .checkers
        .iter()
        .map(|checker| checker.code().len())
        .max()
        .unwrap_or_default();
    for checker in &linter.checkers {
        println!("{:>width$}  {}", checker.code(), checker.name());
    }
}

#[cfg(test)]
mod tests {
    use super::print_checker_list;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let linter = Linter::default();
        print_checker_list(&linter);
        assert!(!linter.checkers.is_empty());
    }
}
