use std::path::Path;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_hit(
    rel_path: &Path,
    hit: &rules::trait_placement::TraitHit,
    project: &Project,
) -> Vec<RuleViolation> {
    let absolute = project.absolute_src_path(rel_path);
    if let Some(target) = hit
        .name
        .strip_suffix("Trait")
        .filter(|bare| !bare.is_empty())
        .map(|bare| rules::trait_placement::TraitPlacement::trait_target_path(rel_path, bare))
    {
        return vec![RuleViolation::from_site(
            absolute,
            hit.line,
            0,
            rules::trait_placement::TraitPlacement::CODE,
            rules::trait_placement::TraitPlacement::suffix_message(&hit.name, &target),
        )];
    }
    let target = rules::trait_placement::TraitPlacement::trait_target_path(rel_path, &hit.name);
    if target == rel_path {
        return Vec::new();
    }
    vec![RuleViolation::from_site(
        absolute,
        hit.line,
        0,
        rules::trait_placement::TraitPlacement::CODE,
        rules::trait_placement::TraitPlacement::misplaced_message(&hit.name, &target),
    )]
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::check_hit;
    use crate::rules::trait_placement::TraitHit;
    use crate::Project;

    fn hit(line: usize, name: &str) -> TraitHit {
        TraitHit {
            line,
            name: name.to_string(),
        }
    }

    #[test]
    fn test_usage() {
        let project = Project::default();
        let violations = check_hit(
            Path::new("stock/replicated_state.rs"),
            &hit(3, "ReplicatedState"),
            &project,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "trait `ReplicatedState` must live in `stock/__traits__/replicated_state.rs`"
        );
    }

    #[test]
    fn test_usage_in_place_trait_is_clean() {
        let project = Project::default();
        let violations = check_hit(
            Path::new("stock/__traits__/sessions.rs"),
            &hit(2, "Sessions"),
            &project,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_suffix_trait_is_flagged() {
        let project = Project::default();
        let violations = check_hit(
            Path::new("stock/__traits__/sessions.rs"),
            &hit(2, "SessionsTrait"),
            &project,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "trait `SessionsTrait` must drop the `Trait` suffix — declare it as `Sessions` so it lives in `stock/__traits__/sessions.rs`"
        );
    }

    #[test]
    fn test_usage_misplaced_suffix_trait_names_its_target() {
        let project = Project::default();
        let violations = check_hit(
            Path::new("room_sync/customer_prefs.rs"),
            &hit(5, "CustomerPrefsTrait"),
            &project,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "trait `CustomerPrefsTrait` must drop the `Trait` suffix — declare it as `CustomerPrefs` so it lives in `room_sync/__traits__/customer_prefs.rs`"
        );
    }
}

// no test_usage necessary
