use crate::ErrorFormat;
use crate::Linter;
use crate::RuleViolation;

pub fn print_violations(linter: &Linter, violations: &[RuleViolation]) {
    let mut stderr = std::io::stderr().lock();
    for d in violations {
        match linter.error_format {
            ErrorFormat::Github => Linter::print_github(d, &mut stderr),
            ErrorFormat::Clippy => Linter::print_clippy(d, &mut stderr),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::print_violations;
    use crate::ErrorFormat;
    use crate::Linter;
    use crate::RuleViolation;

    #[test]
    fn test_usage() {
        let violations = vec![RuleViolation::from_site(
            PathBuf::from("a.rs"),
            1,
            0,
            "LELE001",
            String::from("m"),
        )];
        for error_format in [ErrorFormat::Clippy, ErrorFormat::Github] {
            let linter = Linter {
                rules: Vec::new(),
                error_format,
            };
            print_violations(&linter, &violations);
        }
        assert_eq!(violations.len(), 1);
    }
}
