use crate::rules;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use std::collections::HashSet;

pub struct RootReexport;

impl RootReexport {
    pub const NAME: &'static str = "root_reexport";
    pub const CODE: &'static str = "LELE015";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "A public type in a crate-root file is re-exported from `lib.rs` (`pub use meters::Meters;`); a root fn file is a private `mod` plus `pub use`.",
        why: "Root items are used as `crate::Meters`, never `crate::meters::Meters`.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod meters;
",
            },
            ExampleFile {
                path: "src/meters.rs",
                source: r"use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Meters(pub u32);

#[rustfmt::skip]
impl Meters {
    pub fn doubled(&self) -> u32 { **self * 2 }
}

#[cfg(test)]
mod tests {
    use crate::meters::Meters;

    #[test]
    fn test_usage() {
        assert_eq!(Meters(4).doubled(), 8);
    }
}
",
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod meters;
pub use meters::Meters;
",
            },
            ExampleFile {
                path: "src/meters.rs",
                source: r"use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Meters(pub u32);

#[rustfmt::skip]
impl Meters {
    pub fn doubled(&self) -> u32 { **self * 2 }
}

#[cfg(test)]
mod tests {
    use crate::Meters;

    #[test]
    fn test_usage() {
        assert_eq!(Meters(4).doubled(), 8);
    }
}
",
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for RootReexport {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(RootReexport)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RootReexport {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(RootReexport));
    }
    pub(crate) fn public_modules(lib: &syn::File) -> HashSet<String> {
        lib.items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Mod(m) if matches!(m.vis, syn::Visibility::Public(_)) => {
                    Some(m.ident.to_string())
                }
                _ => None,
            })
            .collect()
    }
}

#[atomic_delegates]
impl RootReexport {
    pub(crate) fn missing_fn_flatten(stem: &str, file: &syn::File, lib: &syn::File) -> Vec<String> {
    }
    pub(crate) fn missing_reexports(
        stem: &str,
        file: &syn::File,
        reexported: &HashSet<(String, String)>,
    ) -> Vec<rules::root_reexport::MissingReexport> {
    }
    pub(crate) fn root_reexports(lib: &syn::File) -> HashSet<(String, String)> {}
}

// no test_usage necessary
