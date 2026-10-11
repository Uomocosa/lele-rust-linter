use crate::print_violations;
use crate::Checker;
use crate::ErrorFormat;
use crate::Project;

pub fn run_checkers(project: &Project, checkers: &[Box<dyn Checker>], format: ErrorFormat) -> i32 {
    let mut all_violations = Vec::new();
    for checker in checkers {
        all_violations.extend(checker.check(project));
    }
    print_violations(&all_violations, format);
    i32::from(!all_violations.is_empty())
}

#[cfg(test)]
mod tests {
    use super::run_checkers;
    use crate::Checker;
    use crate::ErrorFormat;
    use crate::Project;

    #[test]
    fn test_usage() {
        let checkers: Vec<Box<dyn Checker>> = Vec::new();
        assert_eq!(
            run_checkers(&Project::default(), &checkers, ErrorFormat::Clippy),
            0
        );
    }
}
