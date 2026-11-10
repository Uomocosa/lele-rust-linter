use crate::rules;
use crate::Project;
use crate::RuleViolation;
use syn::visit::Visit;

pub fn check(
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

    use super::check;
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
        for violation in check(&TraitImplCoLocation, &project) {
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
    fn test_usage_skips_unresolved_self_types() {
        assert_eq!(
            messages(&[
                ("stock/mod.rs", "pub mod unix_time;\n"),
                (
                    "stock/__traits__/clock.rs",
                    "pub trait Clock { fn now(&self) -> u64; }\n",
                ),
                (
                    "stock/unix_time.rs",
                    "pub struct UnixTime { pub seconds: u64 }\n\nimpl Clock for &UnixTime { fn now(&self) -> u64 { self.seconds } }\n",
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_skips_type_in_basic_container() {
        assert_eq!(
            messages(&[
                ("stock/mod.rs", "pub mod clock;\n"),
                (
                    "stock/__basic__/structs.rs",
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
