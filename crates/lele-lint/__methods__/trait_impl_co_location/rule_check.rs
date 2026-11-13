use crate::rules;
use crate::Project;
use crate::RuleViolation;
use syn::visit::Visit;

pub fn rule_check(
    _self: &rules::trait_impl_co_location::TraitImplCoLocation,
    project: &Project,
) -> Vec<RuleViolation> {
    let type_definitions =
        rules::trait_impl_co_location::TraitImplCoLocation::type_definitions(project);
    let trait_definitions =
        rules::trait_impl_co_location::TraitImplCoLocation::trait_definitions(project);
    let mut violations = Vec::new();

    for (rel_path, file) in &project.parsed_files {
        if rules::trait_impl_co_location::TraitImplCoLocation::is_exempt_source(rel_path) {
            continue;
        }
        let mut hits = Vec::new();
        let mut visitor = rules::trait_impl_co_location::ImplVisitor(&mut hits);
        visitor.visit_file(file);
        for hit in &hits {
            violations.extend(
                rules::trait_impl_co_location::TraitImplCoLocation::check_hit(
                    rel_path,
                    hit,
                    project,
                    &type_definitions,
                    &trait_definitions,
                ),
            );
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::rule_check;
    use crate::rules::trait_impl_co_location::TraitImplCoLocation;
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
        for violation in rule_check(&TraitImplCoLocation, &project) {
            violations.push(violation.message);
        }
        violations.sort();
        violations
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            messages(&[
                ("stock/mod.rs", "pub mod unix_time;\n"),
                (
                    "stock/unix_time.rs",
                    "use crate::stock;\n\npub struct UnixTime { pub seconds: u64 }\n\nimpl stock::Clock for UnixTime { fn now(&self) -> u64 { self.seconds } }\n",
                ),
                (
                    "stock/__traits__/clock.rs",
                    "pub trait Clock { fn now(&self) -> u64; }\n",
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_flags_impl_in_separate_file() {
        let found = messages(&[
            ("stock/mod.rs", "pub mod clock;\npub mod unix_time;\n"),
            (
                "stock/unix_time.rs",
                "pub struct UnixTime { pub seconds: u64 }\n",
            ),
            (
                "stock/__traits__/clock.rs",
                "pub trait Clock { fn now(&self) -> u64; }\n",
            ),
            (
                "stock/clock.rs",
                "use crate::stock;\n\nimpl stock::Clock for UnixTime { fn now(&self) -> u64 { self.seconds } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Clock for UnixTime` must live in `stock/unix_time.rs`, the file that defines `UnixTime`"
        );
    }

    #[test]
    fn test_usage_same_named_types_impl_in_the_defining_file_is_clean() {
        assert_eq!(
            messages(&[
                ("a/mod.rs", "pub mod config;\n"),
                (
                    "a/config.rs",
                    "pub struct Config { pub enabled: bool }\n\nimpl Default for Config { fn default() -> Self { Self { enabled: false } } }\n",
                ),
                ("b/mod.rs", "pub mod resources;\n"),
                (
                    "b/__basic__/resources.rs",
                    "pub struct Config { pub enabled: bool }\n",
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_same_named_types_resolved_through_the_imported_module() {
        let found = messages(&[
            ("a/mod.rs", "pub mod config;\n"),
            ("a/config.rs", "pub struct Config { pub enabled: bool }\n"),
            ("b/mod.rs", "pub mod config;\n"),
            ("b/config.rs", "pub struct Config { pub enabled: bool }\n"),
            ("dial/mod.rs", "pub mod setup;\n"),
            (
                "dial/setup.rs",
                "use crate::a::Config;\n\nimpl Default for Config { fn default() -> Self { Self { enabled: false } } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Default for Config` must live in `a/config.rs`, the file that defines `Config`"
        );
    }

    #[test]
    fn test_usage_same_named_types_without_resolution_is_ambiguous() {
        let found = messages(&[
            ("a/mod.rs", "pub mod config;\n"),
            ("a/config.rs", "pub struct Config { pub enabled: bool }\n"),
            ("b/mod.rs", "pub mod config;\n"),
            ("b/config.rs", "pub struct Config { pub enabled: bool }\n"),
            ("dial/mod.rs", "pub mod setup;\n"),
            (
                "dial/setup.rs",
                "impl Default for Config { fn default() -> Self { Self { enabled: false } } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`Config` is defined in several files (a/config.rs, b/config.rs): move the impl into the one that defines this `Config`"
        );
    }

    #[test]
    fn test_usage_relative_imports_disambiguate_same_named_types() {
        let found = messages(&[
            ("a/mod.rs", "pub mod config;\npub mod setup;\n"),
            ("a/config.rs", "pub struct Config { pub enabled: bool }\n"),
            ("b/mod.rs", "pub mod config;\n"),
            ("b/config.rs", "pub struct Config { pub enabled: bool }\n"),
            (
                "a/setup.rs",
                "use super::config::Config;\n\nimpl Default for Config { fn default() -> Self { Self { enabled: false } } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Default for Config` must live in `a/config.rs`, the file that defines `Config`"
        );
    }

    #[test]
    fn test_usage_flags_blanket_impl_outside_trait_file() {
        let found = messages(&[
            ("stock/mod.rs", "pub mod blanket;\n"),
            (
                "stock/__traits__/clock.rs",
                "pub trait Clock { fn now(&self) -> u64; }\n",
            ),
            (
                "stock/blanket.rs",
                "impl<T> Clock for T { fn now(&self) -> u64 { 0 } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Clock for T` must live in `stock/__traits__/clock.rs`, the file that defines `Clock`"
        );
    }

    #[test]
    fn test_usage_same_named_traits_blanket_impl_resolves_through_the_trait_path() {
        let found = messages(&[
            (
                "a/__traits__/foo.rs",
                "pub trait Foo { fn now(&self) -> u64; }\n",
            ),
            (
                "b/__traits__/foo.rs",
                "pub trait Foo { fn now(&self) -> u64; }\n",
            ),
            (
                "dial/setup.rs",
                "impl crate::a::Foo for T { fn now(&self) -> u64 { 0 } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Foo for T` must live in `a/__traits__/foo.rs`, the file that defines `Foo`"
        );
    }

    #[test]
    fn test_usage_same_named_traits_local_trait_impl_resolves_through_the_trait_path() {
        let found = messages(&[
            (
                "a/__traits__/foo.rs",
                "pub trait Foo { fn now(&self) -> u64; }\n",
            ),
            (
                "b/__traits__/foo.rs",
                "pub trait Foo { fn now(&self) -> u64; }\n",
            ),
            (
                "dial/setup.rs",
                "impl crate::a::Foo for ForeignAlias { fn now(&self) -> u64 { 0 } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Foo for ForeignAlias` must live in `a/__traits__/foo.rs`, the file that defines `Foo`"
        );
    }

    #[test]
    fn test_usage_same_named_traits_resolved_through_the_imported_trait() {
        let found = messages(&[
            (
                "a/__traits__/foo.rs",
                "pub trait Foo { fn now(&self) -> u64; }\n",
            ),
            (
                "b/__traits__/foo.rs",
                "pub trait Foo { fn now(&self) -> u64; }\n",
            ),
            (
                "dial/setup.rs",
                "use crate::b::Foo;\n\nimpl Foo for T { fn now(&self) -> u64 { 0 } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Foo for T` must live in `b/__traits__/foo.rs`, the file that defines `Foo`"
        );
    }

    #[test]
    fn test_usage_ambiguous_same_named_traits_lists_every_defining_file() {
        let found = messages(&[
            (
                "a/__traits__/foo.rs",
                "pub trait Foo { fn now(&self) -> u64; }\n",
            ),
            (
                "b/__traits__/foo.rs",
                "pub trait Foo { fn now(&self) -> u64; }\n",
            ),
            (
                "dial/setup.rs",
                "impl Foo for T { fn now(&self) -> u64 { 0 } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`Foo` is defined in several files (a/__traits__/foo.rs, b/__traits__/foo.rs): move the impl into the one that defines this `Foo`"
        );
    }

    #[test]
    fn test_usage_allows_blanket_impl_in_trait_file() {
        assert_eq!(
            messages(&[
                ("stock/mod.rs", "pub mod unix_time;\n"),
                (
                    "stock/__traits__/clock.rs",
                    "pub trait Clock { fn now(&self) -> u64; }\n\nimpl<T> Clock for Vec<T> { fn now(&self) -> u64 { 0 } }\n",
                ),
                (
                    "stock/unix_time.rs",
                    "pub struct UnixTime { pub seconds: u64 }\n",
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_std_trait_impl_stays_with_type() {
        assert_eq!(
            messages(&[
                ("stock/mod.rs", "pub mod unix_time;\n"),
                (
                    "stock/unix_time.rs",
                    "use std::fmt;\n\npub struct UnixTime { pub seconds: u64 }\n\nimpl fmt::Display for UnixTime { fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { write!(formatter, \"{}\", self.seconds) } }\n",
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_skips_foreign_trait_and_type() {
        assert_eq!(
            messages(&[(
                "stock/foreign.rs",
                "impl Display for ForeignType { fn fmt(&mut self) {} }\n",
            )]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_unresolvable_type_with_local_trait_follows_the_trait() {
        let found = messages(&[
            ("stock/mod.rs", "pub mod unix_time;\n"),
            (
                "stock/__traits__/clock.rs",
                "pub trait Clock { fn now(&self) -> u64; }\n",
            ),
            (
                "stock/unix_time.rs",
                "pub struct UnixTime { pub seconds: u64, pub nanos: u32 }\n",
            ),
            (
                "stock/holder.rs",
                "use crate::stock;\n\nimpl stock::Clock for ForeignAlias { fn now(&self) -> u64 { 0 } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Clock for ForeignAlias` must live in `stock/__traits__/clock.rs`, the file that defines `Clock`"
        );
    }

    #[test]
    fn test_usage_flags_impl_for_container_type() {
        let found = messages(&[
            ("stock/mod.rs", "pub mod clock;\n"),
            (
                "stock/__basic__/structs.rs",
                "pub struct UnixTime { pub seconds: u64, pub nanos: u32 }\n",
            ),
            (
                "stock/__traits__/clock.rs",
                "pub trait Clock { fn now(&self) -> u64; }\n",
            ),
            (
                "stock/clock.rs",
                "use crate::stock;\n\nimpl stock::Clock for UnixTime { fn now(&self) -> u64 { self.seconds } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Clock for UnixTime` lives outside `stock/__basic__/structs.rs`, which defines `UnixTime`: a type with an impl keeps its own file — move `UnixTime` to `stock/unix_time.rs` and put the impl there"
        );
    }

    #[test]
    fn test_usage_container_type_impl_in_same_file_stays_container_placements_call() {
        assert_eq!(
            messages(&[
                ("stock/mod.rs", "pub mod clock;\n"),
                (
                    "stock/__basic__/structs.rs",
                    "pub struct UnixTime { pub seconds: u64, pub nanos: u32 }\n\nimpl stock::Clock for UnixTime { fn now(&self) -> u64 { self.seconds } }\n",
                ),
                (
                    "stock/__traits__/clock.rs",
                    "pub trait Clock { fn now(&self) -> u64; }\n",
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_reference_self_type_resolves_to_the_type() {
        let found = messages(&[
            ("stock/mod.rs", "pub mod unix_time;\n"),
            (
                "stock/unix_time.rs",
                "pub struct UnixTime { pub seconds: u64, pub nanos: u32 }\n",
            ),
            (
                "stock/__traits__/clock.rs",
                "pub trait Clock { fn now(&self) -> u64; }\n",
            ),
            (
                "stock/holder.rs",
                "use crate::stock;\n\nimpl stock::Clock for &UnixTime { fn now(&self) -> u64 { self.seconds } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Clock for UnixTime` must live in `stock/unix_time.rs`, the file that defines `UnixTime`"
        );
    }

    #[test]
    fn test_usage_skips_slice_and_tuple_self_types() {
        assert_eq!(
            messages(&[
                ("stock/mod.rs", "pub mod unix_time;\n"),
                (
                    "stock/unix_time.rs",
                    "pub struct UnixTime { pub seconds: u64, pub nanos: u32 }\n",
                ),
                (
                    "stock/__traits__/clock.rs",
                    "pub trait Clock { fn now(&self) -> u64; }\n",
                ),
                (
                    "stock/holder.rs",
                    "use crate::stock;\n\nimpl stock::Clock for [UnixTime] { fn now(&self) -> u64 { 0 } }\nimpl stock::Clock for (UnixTime, UnixTime) { fn now(&self) -> u64 { 0 } }\n",
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_exempts_test_modules() {
        assert_eq!(
            messages(&[
                ("stock/mod.rs", "pub mod unix_time;\n"),
                (
                    "stock/unix_time.rs",
                    "pub struct UnixTime { pub seconds: u64 }\n\n#[cfg(test)] mod tests { use super::UnixTime; pub trait Local {}\nimpl Local for UnixTime { fn now(&self) -> u64 { 0 } } }\n",
                ),
                (
                    "stock/__traits__/clock.rs",
                    "pub trait Clock { fn now(&self) -> u64; }\n",
                ),
                (
                    "stock/other.rs",
                    "#[cfg(test)] mod tests { use crate::stock::UnixTime; use super::Clock;\nimpl Clock for UnixTime { fn now(&self) -> u64 { 0 } } }\n",
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_exempts_tests_and_build_files() {
        assert_eq!(
            messages(&[
                (
                    "tests/harness.rs",
                    "pub struct UnixTime;\npub trait Clock {}\nimpl Clock for UnixTime {}\n",
                ),
                (
                    "build.rs",
                    "pub struct UnixTime;\npub trait Clock {}\nimpl Clock for UnixTime {}\n",
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_impl_inside_plain_mod_is_still_checked() {
        let found = messages(&[
            ("stock/mod.rs", "pub mod unix_time;\n"),
            (
                "stock/unix_time.rs",
                "pub struct UnixTime { pub seconds: u64 }\n",
            ),
            (
                "stock/__traits__/clock.rs",
                "pub trait Clock { fn now(&self) -> u64; }\n",
            ),
            (
                "stock/holder.rs",
                "#[path = \"holder_inner.rs\"]\nmod holder_inner;\n",
            ),
            (
                "stock/holder_inner.rs",
                "use crate::stock;\n\nimpl stock::Clock for UnixTime { fn now(&self) -> u64 { self.seconds } }\n",
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`impl Clock for UnixTime` must live in `stock/unix_time.rs`, the file that defines `UnixTime`"
        );
    }
}

// no test_usage necessary
