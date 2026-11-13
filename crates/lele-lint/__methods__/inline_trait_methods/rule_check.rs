use crate::rules;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::inline_trait_methods::InlineTraitMethods,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations =
        rules::inline_trait_methods::InlineTraitMethods::config_violations(project);
    for (rel_path, file) in &project.parsed_files {
        for item in &file.items {
            let syn::Item::Impl(impl_block) = item else {
                continue;
            };
            if impl_block
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            violations.extend(
                rules::inline_trait_methods::InlineTraitMethods::impl_block_violations(
                    project, rel_path, file, impl_block,
                ),
            );
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::rule_check;
    use crate::rules;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project::default();
        assert!(rule_check(&rules::inline_trait_methods::InlineTraitMethods, &project).is_empty());
    }
}
