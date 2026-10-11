use crate::ErrorFormat;
use crate::Location;
use crate::RuleViolation;
use std::io::Write;

pub fn print_violations(violations: &[RuleViolation], error_format: ErrorFormat) {
    let mut stderr = std::io::stderr().lock();
    for d in violations {
        match error_format {
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

// no test_usage necessary
