use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &rules::no_comments::NoComments, project: &Project) -> Vec<RuleViolation> {
    let mut violations =
        rules::no_comments::NoComments::scan_entries(&project.entries, &project.src_dir);
    if let Some(methods_dir) = &project.methods_dir {
        violations.extend(rules::no_comments::NoComments::scan_entries(
            &project.methods_entries,
            methods_dir,
        ));
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::rules;
    use crate::Project;

    #[test]
    fn test_usage() {
        let rule = rules::no_comments::NoComments;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }
}
