use crate::config;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn config_violations(project: &Project) -> Vec<RuleViolation> {
    let code = rules::inline_trait_methods::InlineTraitMethods::CODE;
    let config_path = project.config_base().join(config::CONFIG_FILENAME);
    let mut violations = Vec::new();
    for entry in &project.inline_trait_methods {
        let trait_path = &entry.trait_path;
        if entry.reason.trim().is_empty() {
            violations.push(RuleViolation::from_file(
                config_path.clone(),
                code,
                format!(
                    "[[lele.lint.inline_trait_methods]] entry for `{trait_path}` has no reason"
                ),
            ));
        }
        if entry.methods.is_empty() {
            violations.push(RuleViolation::from_file(
                config_path.clone(),
                code,
                format!(
                    "[[lele.lint.inline_trait_methods]] entry for `{trait_path}` lists no methods"
                ),
            ));
        }
        for (index, method) in entry.methods.iter().enumerate() {
            if entry
                .methods
                .iter()
                .take(index)
                .any(|earlier| earlier == method)
            {
                violations.push(RuleViolation::from_file(
                    config_path.clone(),
                    code,
                    format!("[[lele.lint.inline_trait_methods]] entry for `{trait_path}` lists `{method}` twice"),
                ));
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::InlineTraitMethodsEntry;
    use crate::Project;

    #[test]
    fn test_usage() {
        let entry = |methods: &[&str], reason: &str| InlineTraitMethodsEntry {
            trait_path: "bevy::app::Plugin".to_string(),
            methods: methods.iter().map(|method| (*method).to_string()).collect(),
            reason: reason.to_string(),
        };
        let project = Project {
            inline_trait_methods: vec![
                entry(&["build"], " "),
                entry(&[], "framework reads the impl"),
                entry(&["build", "ready", "build"], "framework reads the impl"),
            ],
            ..Project::default()
        };
        let violations =
            rules::inline_trait_methods::InlineTraitMethods::config_violations(&project);
        assert_eq!(violations.len(), 3);
        assert!(violations[0].message.contains("has no reason"));
        assert!(violations[1].message.contains("lists no methods"));
        assert!(violations[2].message.contains("lists `build` twice"));
    }
}
