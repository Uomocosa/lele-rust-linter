use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(_self: &rules::no_comments::NoComments, project: &Project) -> Vec<RuleViolation> {
    let mut violations = rules::no_comments::NoComments::scan_entries(&project.entries);
    violations.extend(rules::no_comments::NoComments::scan_entries(
        &project.methods_entries,
    ));
    violations
}

#[cfg(test)]
mod tests {
    use super::rule_check;
    use crate::rules;
    use crate::Project;

    #[test]
    fn test_usage() {
        let rule = rules::no_comments::NoComments;
        let project = Project::default();
        assert!(rule_check(&rule, &project).is_empty());
    }
}
