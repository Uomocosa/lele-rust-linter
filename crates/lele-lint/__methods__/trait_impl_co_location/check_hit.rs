use std::path::Path;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_hit(
    rel_path: &Path,
    hit: &rules::trait_impl_co_location::ImplHit,
    project: &Project,
    type_definitions: &rules::trait_impl_co_location::DefinitionMap,
    trait_definitions: &rules::trait_impl_co_location::DefinitionMap,
) -> Vec<RuleViolation> {
    let Some(type_name) = &hit.type_name else {
        return Vec::new();
    };
    if !hit.is_blanket {
        if let Some(files) = type_definitions.get(type_name) {
            let Some(target) = files.iter().next() else {
                return Vec::new();
            };
            if target == rel_path {
                return Vec::new();
            }
            if project.is_in_dunder_dir(target) {
                let graduate =
                    rules::trait_impl_co_location::TraitImplCoLocation::type_graduate_target(
                        target, type_name,
                    );
                return vec![RuleViolation::from_site(
                    project.absolute_src_path(rel_path),
                    hit.line,
                    0,
                    rules::trait_impl_co_location::TraitImplCoLocation::CODE,
                    rules::trait_impl_co_location::TraitImplCoLocation::container_target_message(
                        &hit.trait_name,
                        type_name,
                        target,
                        &graduate,
                    ),
                )];
            }
            return vec![RuleViolation::from_site(
                project.absolute_src_path(rel_path),
                hit.line,
                0,
                rules::trait_impl_co_location::TraitImplCoLocation::CODE,
                rules::trait_impl_co_location::TraitImplCoLocation::impl_target_message(
                    &hit.trait_name,
                    type_name,
                    target,
                ),
            )];
        }
    }
    let Some(target) = trait_definitions
        .get(&hit.trait_name)
        .and_then(|files| files.iter().next())
    else {
        return Vec::new();
    };
    if target == rel_path {
        return Vec::new();
    }
    vec![RuleViolation::from_site(
        project.absolute_src_path(rel_path),
        hit.line,
        0,
        rules::trait_impl_co_location::TraitImplCoLocation::CODE,
        rules::trait_impl_co_location::TraitImplCoLocation::trait_target_message(
            &hit.trait_name,
            type_name,
            target,
        ),
    )]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::path::Path;
    use std::path::PathBuf;

    use super::check_hit;
    use crate::rules;
    use crate::DunderPath;
    use crate::Project;

    fn insert(definitions: &mut BTreeMap<String, BTreeSet<PathBuf>>, rel_path: &str, name: &str) {
        definitions
            .entry(name.to_string())
            .or_default()
            .insert(PathBuf::from(rel_path));
    }

    fn trait_hit(
        line: usize,
        trait_name: &str,
        type_name: &str,
    ) -> rules::trait_impl_co_location::ImplHit {
        rules::trait_impl_co_location::ImplHit {
            line,
            trait_name: trait_name.to_string(),
            type_name: Some(type_name.to_string()),
            is_blanket: false,
        }
    }

    fn blanket_hit(line: usize, trait_name: &str) -> rules::trait_impl_co_location::ImplHit {
        rules::trait_impl_co_location::ImplHit {
            line,
            trait_name: trait_name.to_string(),
            type_name: Some("T".to_string()),
            is_blanket: true,
        }
    }

    fn unnamed_hit(line: usize, trait_name: &str) -> rules::trait_impl_co_location::ImplHit {
        rules::trait_impl_co_location::ImplHit {
            line,
            trait_name: trait_name.to_string(),
            type_name: None,
            is_blanket: false,
        }
    }

    fn project() -> Project {
        Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        }
    }

    fn insert_definitions(
        type_definitions: &mut BTreeMap<String, BTreeSet<PathBuf>>,
        trait_definitions: &mut BTreeMap<String, BTreeSet<PathBuf>>,
    ) {
        insert(type_definitions, "stock/unix_time.rs", "UnixTime");
        insert(type_definitions, "stock/wrapper.rs", "Wrapper");
        insert(type_definitions, "stock/__basic__/structs.rs", "BasicOnly");
        insert(trait_definitions, "stock/__traits__/clock.rs", "Clock");
    }

    #[test]
    fn test_usage() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/clock.rs"),
            &trait_hit(3, "Clock", "UnixTime"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Clock for UnixTime` must live in `stock/unix_time.rs`, the file that defines `UnixTime`"
        );
    }

    #[test]
    fn test_usage_in_place_impl_is_clean() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/unix_time.rs"),
            &trait_hit(5, "Clock", "UnixTime"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_generic_wrapper_resolves_last_segment() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/clock.rs"),
            &trait_hit(3, "Clock", "Wrapper"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Clock for Wrapper` must live in `stock/wrapper.rs`, the file that defines `Wrapper`"
        );
    }

    #[test]
    fn test_usage_blanket_impl_follows_the_trait() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/blanket.rs"),
            &blanket_hit(3, "Clock"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Clock for T` must live in `stock/__traits__/clock.rs`, the file that defines `Clock`"
        );
    }

    #[test]
    fn test_usage_blanket_impl_in_trait_file_is_clean() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/__traits__/clock.rs"),
            &blanket_hit(5, "Clock"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_foreign_type_with_local_trait_follows_the_trait() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/foreign.rs"),
            &trait_hit(2, "Clock", "ForeignType"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Clock for ForeignType` must live in `stock/__traits__/clock.rs`, the file that defines `Clock`"
        );
    }

    #[test]
    fn test_usage_foreign_trait_and_type_is_skipped() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/foreign.rs"),
            &trait_hit(2, "Display", "ForeignType"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_unnamed_self_type_is_skipped() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/foreign.rs"),
            &unnamed_hit(2, "Display"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_container_type_demands_graduation() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/clock.rs"),
            &trait_hit(3, "Clock", "BasicOnly"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Clock for BasicOnly` lives outside `stock/__basic__/structs.rs`, which defines `BasicOnly`: a type with an impl keeps its own file — move `BasicOnly` to `stock/basic_only.rs` and put the impl there"
        );
    }

    #[test]
    fn test_usage_local_type_beats_local_trait() {
        let project = project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        insert_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/__traits__/clock.rs"),
            &trait_hit(6, "Clock", "UnixTime"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Clock for UnixTime` must live in `stock/unix_time.rs`, the file that defines `UnixTime`"
        );
    }
}

// no test_usage necessary
