use crate::Checker;

pub fn print_checker_list(checkers: &[Box<dyn Checker>]) {
    let width = checkers
        .iter()
        .map(|checker| checker.code().len())
        .max()
        .unwrap_or_default();
    for checker in checkers {
        println!("{:>width$}  {}", checker.code(), checker.name());
    }
}

// no test_usage necessary
