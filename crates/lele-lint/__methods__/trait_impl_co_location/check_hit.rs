use std::path::Path;
use std::path::PathBuf;

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
            if files.contains(rel_path) {
                return Vec::new();
            }
            let hints = rules::trait_impl_co_location::TraitImplCoLocation::module_hints(
                rel_path,
                type_name,
                hit.type_path.as_deref(),
                project,
            );
            let narrowed = rules::trait_impl_co_location::TraitImplCoLocation::narrowed_candidates(
                files, &hints,
            );
            let resolved: Option<&Path> = if files.len() == 1 {
                files.iter().next().map(PathBuf::as_path)
            } else {
                match narrowed.as_slice() {
                    [single] => Some(single.as_path()),
                    _ => None,
                }
            };
            if let Some(target) = resolved {
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
            if files.len() > 1 {
                return vec![RuleViolation::from_site(
                    project.absolute_src_path(rel_path),
                    hit.line,
                    0,
                    rules::trait_impl_co_location::TraitImplCoLocation::CODE,
                    rules::trait_impl_co_location::TraitImplCoLocation::ambiguous_target_message(
                        type_name, files,
                    ),
                )];
            }
            return Vec::new();
        }
    }
    let Some(trait_files) = trait_definitions.get(&hit.trait_name) else {
        return Vec::new();
    };
    if trait_files.contains(rel_path) {
        return Vec::new();
    }
    let hints = rules::trait_impl_co_location::TraitImplCoLocation::module_hints(
        rel_path,
        &hit.trait_name,
        hit.trait_path.as_deref(),
        project,
    );
    let narrowed = rules::trait_impl_co_location::TraitImplCoLocation::narrowed_candidates(
        trait_files,
        &hints,
    );
    let trait_target: Option<&Path> = if trait_files.len() == 1 {
        trait_files.iter().next().map(PathBuf::as_path)
    } else {
        match narrowed.as_slice() {
            [single] => Some(single.as_path()),
            _ => None,
        }
    };
    match trait_target {
        Some(target) => {
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
        None if trait_files.len() > 1 => {
            vec![RuleViolation::from_site(
                project.absolute_src_path(rel_path),
                hit.line,
                0,
                rules::trait_impl_co_location::TraitImplCoLocation::CODE,
                rules::trait_impl_co_location::TraitImplCoLocation::ambiguous_target_message(
                    &hit.trait_name,
                    trait_files,
                ),
            )]
        }
        None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::Path;

    use super::check_hit;
    use crate::testing;

    #[test]
    fn test_usage() {
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/clock.rs"),
            &testing::named_impl_hit(3, "Clock", "UnixTime"),
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
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/unix_time.rs"),
            &testing::named_impl_hit(5, "Clock", "UnixTime"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_same_named_types_impl_in_the_defining_file_is_clean() {
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let trait_definitions = BTreeMap::new();
        testing::same_named_config_definitions(&mut type_definitions);
        let violations = check_hit(
            Path::new("discovery/config.rs"),
            &testing::named_impl_hit(3, "Default", "Config"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_same_named_types_resolved_through_an_import() {
        let project = testing::parsed_project(
            testing::dunder_project(),
            &[(
                "dial/setup.rs",
                "use crate::discovery::Config;\n\nimpl Default for Config {}",
            )],
        );
        let mut type_definitions = BTreeMap::new();
        let trait_definitions = BTreeMap::new();
        testing::same_named_config_definitions(&mut type_definitions);
        let violations = check_hit(
            Path::new("dial/setup.rs"),
            &testing::named_impl_hit(3, "Default", "Config"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Default for Config` must live in `discovery/config.rs`, the file that defines `Config`"
        );
    }

    #[test]
    fn test_usage_super_import_selects_the_defining_file() {
        let project = testing::parsed_project(
            testing::dunder_project(),
            &[(
                "discovery/setup.rs",
                "use super::config::Config;\n\nimpl Default for Config {}",
            )],
        );
        let mut type_definitions = BTreeMap::new();
        let trait_definitions = BTreeMap::new();
        testing::same_named_config_definitions(&mut type_definitions);
        let violations = check_hit(
            Path::new("discovery/setup.rs"),
            &testing::named_impl_hit(3, "Default", "Config"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Default for Config` must live in `discovery/config.rs`, the file that defines `Config`"
        );
    }

    #[test]
    fn test_usage_dunder_trait_import_resolves_through_the_mount() {
        let project = testing::parsed_project(
            testing::dunder_project(),
            &[(
                "dial/setup.rs",
                "use crate::stock::traits::Foo;\n\nimpl Foo for T {}",
            )],
        );
        let mut trait_definitions = BTreeMap::new();
        testing::define_type(&mut trait_definitions, "a/__traits__/foo.rs", "Foo");
        testing::define_type(&mut trait_definitions, "stock/__traits__/foo.rs", "Foo");
        let violations = check_hit(
            Path::new("dial/setup.rs"),
            &testing::blanket_impl_hit(3, "Foo"),
            &project,
            &BTreeMap::new(),
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Foo for T` must live in `stock/__traits__/foo.rs`, the file that defines `Foo`"
        );
    }

    #[test]
    fn test_usage_same_named_types_resolved_through_the_named_self_path() {
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let trait_definitions = BTreeMap::new();
        testing::same_named_config_definitions(&mut type_definitions);
        let violations = check_hit(
            Path::new("dial/setup.rs"),
            &testing::qualified_impl_hit(3, "Default", &["crate", "direct_dial", "Config"]),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Default for Config` lives outside `direct_dial/__basic__/resources.rs`, which defines `Config`: a type with an impl keeps its own file — move `Config` to `direct_dial/config.rs` and put the impl there"
        );
    }

    #[test]
    fn test_usage_ambiguous_same_named_types_lists_every_defining_file() {
        let project = testing::parsed_project(
            testing::dunder_project(),
            &[(
                "dial/setup.rs",
                "use std::fmt;\n\nimpl Default for Config {}",
            )],
        );
        let mut type_definitions = BTreeMap::new();
        let trait_definitions = BTreeMap::new();
        testing::same_named_config_definitions(&mut type_definitions);
        let violations = check_hit(
            Path::new("dial/setup.rs"),
            &testing::named_impl_hit(3, "Default", "Config"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`Config` is defined in several files (direct_dial/__basic__/resources.rs, discovery/config.rs): move the impl into the one that defines this `Config`"
        );
    }

    #[test]
    fn test_usage_generic_wrapper_resolves_last_segment() {
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/clock.rs"),
            &testing::named_impl_hit(3, "Clock", "Wrapper"),
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
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/blanket.rs"),
            &testing::blanket_impl_hit(3, "Clock"),
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
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/__traits__/clock.rs"),
            &testing::blanket_impl_hit(5, "Clock"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_same_named_traits_blanket_impl_in_the_defining_trait_file_is_clean() {
        let project = testing::dunder_project();
        let trait_definitions = testing::same_named_trait_map();
        let violations = check_hit(
            Path::new("a/__traits__/foo.rs"),
            &testing::blanket_impl_hit(3, "Foo"),
            &project,
            &BTreeMap::new(),
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_same_named_traits_resolved_through_the_qualified_trait_path() {
        let project = testing::dunder_project();
        let trait_definitions = testing::same_named_trait_map();
        let violations = check_hit(
            Path::new("dial/setup.rs"),
            &testing::qualified_trait_impl_hit(3, &["crate", "a", "Foo"]),
            &project,
            &BTreeMap::new(),
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Foo for T` must live in `a/__traits__/foo.rs`, the file that defines `Foo`"
        );
    }

    #[test]
    fn test_usage_same_named_traits_resolved_through_an_import() {
        let project = testing::parsed_project(
            testing::dunder_project(),
            &[("dial/setup.rs", "use crate::b::Foo;\n\nimpl Foo for T {}")],
        );
        let trait_definitions = testing::same_named_trait_map();
        let violations = check_hit(
            Path::new("dial/setup.rs"),
            &testing::blanket_impl_hit(3, "Foo"),
            &project,
            &BTreeMap::new(),
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`impl Foo for T` must live in `b/__traits__/foo.rs`, the file that defines `Foo`"
        );
    }

    #[test]
    fn test_usage_ambiguous_same_named_traits_lists_every_defining_file() {
        let project = testing::parsed_project(
            testing::dunder_project(),
            &[("dial/setup.rs", "use std::fmt;\n\nimpl Foo for T {}")],
        );
        let trait_definitions = testing::same_named_trait_map();
        let violations = check_hit(
            Path::new("dial/setup.rs"),
            &testing::blanket_impl_hit(3, "Foo"),
            &project,
            &BTreeMap::new(),
            &trait_definitions,
        );
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].message,
            "`Foo` is defined in several files (a/__traits__/foo.rs, b/__traits__/foo.rs): move the impl into the one that defines this `Foo`"
        );
    }

    #[test]
    fn test_usage_foreign_type_with_local_trait_follows_the_trait() {
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/foreign.rs"),
            &testing::named_impl_hit(2, "Clock", "ForeignType"),
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
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/foreign.rs"),
            &testing::named_impl_hit(2, "Display", "ForeignType"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_unnamed_self_type_is_skipped() {
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/foreign.rs"),
            &testing::unnamed_impl_hit(2, "Display"),
            &project,
            &type_definitions,
            &trait_definitions,
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_container_type_demands_graduation() {
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/clock.rs"),
            &testing::named_impl_hit(3, "Clock", "BasicOnly"),
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
        let project = testing::dunder_project();
        let mut type_definitions = BTreeMap::new();
        let mut trait_definitions = BTreeMap::new();
        testing::stock_definitions(&mut type_definitions, &mut trait_definitions);
        let violations = check_hit(
            Path::new("stock/__traits__/clock.rs"),
            &testing::named_impl_hit(6, "Clock", "UnixTime"),
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
