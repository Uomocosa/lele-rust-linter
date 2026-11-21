use crate::Linter;
use crate::Project;

pub fn run(linter: &Linter, project: &Project) -> i32 {
    let all_violations = linter.violations(project);
    linter.print_violations(&all_violations);
    i32::from(!all_violations.is_empty())
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::ErrorFormat;
    use crate::Linter;
    use crate::Project;

    #[test]
    fn test_usage() {
        let linter = Linter {
            rules: Vec::new(),
            error_format: ErrorFormat::Clippy,
            exemptions: Vec::new(),
        };
        assert_eq!(run(&linter, &Project::default()), 0);
    }
}
