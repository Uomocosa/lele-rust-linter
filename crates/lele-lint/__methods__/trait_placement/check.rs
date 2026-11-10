use syn::visit::Visit;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::trait_placement::TraitPlacement,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for (rel_path, file) in &project.parsed_files {
        if rules::trait_placement::TraitPlacement::is_exempt_source(rel_path) {
            continue;
        }
        let mut hits = Vec::new();
        let mut visitor = rules::trait_placement::TraitVisitor(&mut hits);
        visitor.visit_file(file);
        for hit in &hits {
            violations.extend(rules::trait_placement::TraitPlacement::check_hit(
                rel_path, hit, project,
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules::trait_placement::TraitPlacement;
    use crate::DunderPath;
    use crate::Project;

    fn messages(files: &[(&str, &str)]) -> Vec<String> {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        let mut violations = Vec::new();
        for (path, source) in files {
            let file: syn::File = syn::parse_str(source).unwrap();
            project.parsed_files.insert(PathBuf::from(path), file);
        }
        for violation in check(&TraitPlacement, &project) {
            violations.push(violation.message);
        }
        violations.sort();
        violations
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            messages(&[
                ("stock/mod.rs", "pub mod traits;\n"),
                ("stock/__traits__/sessions.rs", "pub trait Sessions {}\n"),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_flags_trait_in_plain_file() {
        let found = messages(&[(
            "room_sync/replicated_state.rs",
            "pub trait ReplicatedState {}\n",
        )]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "trait `ReplicatedState` must live in `room_sync/__traits__/replicated_state.rs`"
        );
    }

    #[test]
    fn test_usage_flags_trait_suffix() {
        let found = messages(&[("stock/sessions.rs", "pub trait SessionsTrait {}\n")]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "trait `SessionsTrait` must drop the `Trait` suffix — declare it as `Sessions` so it lives in `stock/__traits__/sessions.rs`"
        );
    }

    #[test]
    fn test_usage_flags_misnamed_trait_file_inside_traits_dir() {
        let found = messages(&[("stock/__traits__/water.rs", "pub trait Sessions {}\n")]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "trait `Sessions` must live in `stock/__traits__/sessions.rs`"
        );
    }

    #[test]
    fn test_usage_exempts_test_modules() {
        assert_eq!(
            messages(&[(
                "stock/sessions.rs",
                "#[cfg(test)] mod tests { pub trait TestOnly {} }\n",
            )]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_exempts_tests_and_build_files() {
        assert_eq!(
            messages(&[
                ("tests/harness.rs", "pub trait TestHarness {}\n"),
                ("build.rs", "pub trait BuildHook {}\n"),
            ]),
            Vec::<String>::new()
        );
    }
}

// no test_usage necessary
