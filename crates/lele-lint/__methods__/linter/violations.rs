use crate::Linter;
use crate::Project;
use crate::RuleViolation;

pub fn violations(linter: &Linter, project: &Project) -> Vec<RuleViolation> {
    let mut all_violations = Vec::new();
    for rule in &linter.rules {
        all_violations.extend(rule.check(project));
    }
    let mut match_counts = vec![0_usize; linter.exemptions.len()];
    for violation in &all_violations {
        for (count, exemption) in match_counts.iter_mut().zip(linter.exemptions.iter()) {
            if exemption.matches(violation, project) {
                *count = count.saturating_add(1);
            }
        }
    }
    all_violations.retain(|violation| {
        !linter
            .exemptions
            .iter()
            .any(|exemption| exemption.matches(violation, project))
    });
    for (exemption, count) in linter.exemptions.iter().zip(match_counts.iter()) {
        if *count != 0 {
            continue;
        }
        if !linter
            .rules
            .iter()
            .any(|rule| rule.code() == exemption.code)
        {
            continue;
        }
        let joined = project.config_base().join(&exemption.root);
        if !Project::lexical_absolute(&joined).starts_with(Project::lexical_absolute(&project.root))
        {
            continue;
        }
        let Some(rule) = linter
            .rules
            .iter()
            .find(|rule| rule.code() == exemption.code)
        else {
            continue;
        };
        all_violations.push(RuleViolation::from_file(
            joined,
            rule.code(),
            format!(
                "lele.toml exemption for `{}` under `{}` suppresses nothing; remove it",
                exemption.code,
                exemption.root.display()
            ),
        ));
    }
    all_violations
}

#[cfg(test)]
mod tests {
    use super::violations;
    use crate::ErrorFormat;
    use crate::Linter;
    use crate::PathExemption;
    use crate::Project;

    #[test]
    fn test_usage() {
        let linter = Linter {
            rules: Vec::new(),
            error_format: ErrorFormat::Clippy,
            exemptions: Vec::new(),
        };
        assert!(violations(&linter, &Project::default()).is_empty());
    }

    #[test]
    fn stale_exemption_under_root_is_reported() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            ..Project::default()
        };
        let linter = Linter::default();
        let code = linter
            .rules
            .iter()
            .find(|rule| rule.check(&project).is_empty())
            .map(|rule| rule.code().to_owned())
            .unwrap();
        let linter = Linter {
            rules: linter.rules,
            error_format: ErrorFormat::Clippy,
            exemptions: vec![PathExemption {
                code: code.clone(),
                root: std::path::PathBuf::from("src/vanished"),
                reason: String::from("nothing here yet"),
                replaced_by: None,
            }],
        };
        let found = violations(&linter, &project);
        let stale: Vec<_> = found
            .iter()
            .filter(|violation| violation.message.contains("suppresses nothing"))
            .collect();
        assert_eq!(stale.len(), 1, "{found:#?}");
        assert_eq!(stale[0].code, code.as_str());
        assert_eq!(stale[0].file, dir.path().join("src/vanished"));
    }

    #[test]
    fn stale_exemption_outside_root_is_not_reported() {
        let dir = tempfile::tempdir().unwrap();
        let elsewhere = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            ..Project::default()
        };
        let linter = Linter::default();
        let code = linter
            .rules
            .first()
            .map(|rule| rule.code().to_owned())
            .unwrap();
        let linter = Linter {
            rules: linter.rules,
            error_format: ErrorFormat::Clippy,
            exemptions: vec![PathExemption {
                code,
                root: elsewhere.path().join("sub"),
                reason: String::from("another crate"),
                replaced_by: None,
            }],
        };
        let found = violations(&linter, &project);
        assert!(
            found
                .iter()
                .all(|violation| !violation.message.contains("suppresses nothing")),
            "{found:#?}"
        );
    }
}
