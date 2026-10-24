use crate::rules;
use crate::BoundaryEntry;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

pub struct BoundaryImports;

impl BoundaryImports {
    pub const NAME: &'static str = "boundary_imports";
    pub const CODE: &'static str = "LELE033";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "A `[[lele.boundary]]` folder may not use any path from its `cannot_use` list.",
        why: "A boundary marks a pure core; forbidden imports are how I/O and frameworks leak in.",
        bad: &[
            ExampleFile {
                path: "lele.toml",
                source: r#"[[lele.boundary]]
name = "session is pure"
why = "no io"
folders = ["src/session"]
cannot_use = ["std::time"]
require = "honest"
"#,
            },
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod session;
",
            },
            ExampleFile {
                path: "src/session/mod.rs",
                source: r"mod session;
pub use session::Session;
",
            },
            ExampleFile {
                path: "src/session/session.rs",
                source: r"use derive_more::Deref;

#[derive(Deref)]
pub struct Session(pub u64);

#[rustfmt::skip]
impl Session {
    pub fn expiry(&self) -> u64 { std::time::SystemTime::now().elapsed().map_or(**self, |_| self.saturating_sub(1)) }
}
",
            },
        ],
        good: &[
            ExampleFile {
                path: "lele.toml",
                source: r#"[[lele.boundary]]
name = "session is pure"
why = "no io"
folders = ["src/session"]
cannot_use = ["std::time"]
require = "honest"
"#,
            },
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod session;
",
            },
            ExampleFile {
                path: "src/session/mod.rs",
                source: r"mod session;
pub use session::Session;
",
            },
            ExampleFile {
                path: "src/session/session.rs",
                source: r"use derive_more::Deref;

#[derive(Deref)]
pub struct Session(pub u64);

#[rustfmt::skip]
impl Session {
    pub fn expiry(&self) -> u64 { self.saturating_sub(1) }
}

#[cfg(test)]
mod tests {
    use super::Session;

    #[test]
    fn test_usage() {
        assert_eq!(Session(3).expiry(), 2);
    }
}
",
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for BoundaryImports {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BoundaryImports)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BoundaryImports {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(BoundaryImports));
    }
    pub(crate) fn src_stem_map(project: &Project) -> HashMap<String, PathBuf> {
        project
            .parsed_files
            .keys()
            .filter_map(|rel| {
                rel.file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|stem| (stem.to_string(), rel.clone()))
            })
            .collect()
    }
    pub(crate) fn is_local(path: &str) -> bool {
        ["crate", "self", "super"]
            .iter()
            .any(|root| path == *root || path.starts_with(&format!("{root}::")))
    }
    pub(crate) fn forbidden_hit(hit: &rules::boundary_imports::PathHit, cannot_use: &[String]) -> Option<String> {
        if cannot_use
            .iter()
            .any(|forbidden| Self::matches_hit(forbidden, hit))
        {
            Some(hit.path.clone())
        } else {
            None
        }
    }
    pub(crate) fn walk_use_tree(
        tree: &syn::UseTree,
        prefix: &mut Vec<String>,
        collector: &mut rules::boundary_imports::UseCollector,
        line: usize,
    ) {
        match tree {
            syn::UseTree::Path(path) => {
                let mut next = prefix.clone();
                next.push(path.ident.to_string());
                Self::walk_use_tree(&path.tree, &mut next, collector, line);
            }
            syn::UseTree::Name(name) => {
                let mut full = prefix.clone();
                if name.ident != "self" {
                    full.push(name.ident.to_string());
                }
                let joined = full.join("::");
                collector
                    .aliases
                    .insert(name.ident.to_string(), joined.clone());
                collector.hits.push(rules::boundary_imports::PathHit {
                    path: joined,
                    line,
                    glob: false,
                });
            }
            syn::UseTree::Rename(rename) => {
                let mut full = prefix.clone();
                full.push(rename.ident.to_string());
                let joined = full.join("::");
                collector
                    .aliases
                    .insert(rename.rename.to_string(), joined.clone());
                collector.hits.push(rules::boundary_imports::PathHit {
                    path: joined,
                    line,
                    glob: false,
                });
            }
            syn::UseTree::Glob(_) => {
                collector.hits.push(rules::boundary_imports::PathHit {
                    path: prefix.join("::"),
                    line,
                    glob: true,
                });
            }
            syn::UseTree::Group(group) => {
                for item in &group.items {
                    Self::walk_use_tree(item, prefix, collector, line);
                }
            }
        }
    }
}

#[atomic_delegates]
impl BoundaryImports {
    pub(crate) fn base_prefix(project: &Project, base: &Path) -> PathBuf {}
    pub(crate) fn file_in_boundary(
        crate_rel: &Path,
        prefix: &Path,
        folders: &[PathBuf],
        stem_map: &HashMap<String, PathBuf>,
    ) -> bool {
    }
    pub(crate) fn validate_boundary(boundary: &BoundaryEntry, base: &Path) -> Vec<RuleViolation> {}
    pub(crate) fn collect_file_uses(project: &Project) -> Vec<rules::boundary_imports::FileUse> {}
    pub(crate) fn file_use(
        file: PathBuf,
        crate_rel: PathBuf,
        parsed: &syn::File,
    ) -> rules::boundary_imports::FileUse {
    }
    pub(crate) fn report_uses(
        file: &rules::boundary_imports::FileUse,
        boundary: &BoundaryEntry,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn use_violation(
        file: &Path,
        line: usize,
        reported: &str,
        boundary: &BoundaryEntry,
    ) -> RuleViolation {
    }
    pub(crate) fn matches_hit(forbidden: &str, hit: &rules::boundary_imports::PathHit) -> bool {}
    pub(crate) fn expand(raw: &str, aliases: &HashMap<String, String>) -> String {}
}

// no test_usage necessary
