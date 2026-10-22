use crate::ErrorFormat;
use crate::Linter;
use crate::Location;
use crate::RuleViolation;
use std::io::Write;

pub fn print_violations(linter: &Linter, violations: &[RuleViolation]) {
    let mut stderr = std::io::stderr().lock();
    for d in violations {
        match linter.error_format {
            ErrorFormat::Github => print_github(d, &mut stderr),
            ErrorFormat::Clippy => print_clippy(d, &mut stderr),
        }
    }
}

fn print_clippy(d: &RuleViolation, w: &mut impl Write) {
    let _ = match d.location {
        Location::File => writeln!(w, "{}: error[{}]: {}", d.file.display(), d.code, d.message),
        Location::Site { line, col } => writeln!(
            w,
            "{}:{}:{}: error[{}]: {}",
            d.file.display(),
            line,
            col,
            d.code,
            d.message
        ),
    };
}

fn print_github(d: &RuleViolation, w: &mut impl Write) {
    let _ = match d.location {
        Location::File => writeln!(
            w,
            "::error file={file},line=1,title={code}::{message}",
            file = d.file.display(),
            code = d.code,
            message = d.message
        ),
        Location::Site { line, col } => writeln!(
            w,
            "::error file={file},line={line},col={col},title={code}::{message}",
            file = d.file.display(),
            code = d.code,
            message = d.message
        ),
    };
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
                checkers: Vec::new(),
                error_format,
            };
            print_violations(&linter, &violations);
        }
        assert_eq!(violations.len(), 1);
    }
}
