use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct NoSuperImports;

impl NoSuperImports {
    pub const NAME: &'static str = "no_super_imports";
    pub const CODE: &'static str = "LELE013";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "`super::` is allowed only inside `#[cfg(test)]`; production code imports the domain (`use crate::stock;`).",
        why: "`super::` paths break when files move; domain paths read the same from every file.",
        bad: rule_examples!("no_super_imports/bad"),
        good: rule_examples!("no_super_imports/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoSuperImports {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoSuperImports)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoSuperImports {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoSuperImports));
    }
    pub(crate) fn generic_suggestion() -> (String, String) {
        (
            "use crate::<domain>;".to_string(),
            "<domain>::…".to_string(),
        )
    }
    pub(crate) fn starts_with_super(tree: &syn::UseTree) -> bool {
        matches!(tree, syn::UseTree::Path(path) if path.ident == "super")
    }
    pub(crate) fn use_tree_string(tree: &syn::UseTree) -> String {
        match tree {
            syn::UseTree::Path(path) => format!("{}::{}", path.ident, Self::use_tree_string(&path.tree)),
            syn::UseTree::Name(name) => name.ident.to_string(),
            syn::UseTree::Rename(rename) => format!("{} as {}", rename.ident, rename.rename),
            syn::UseTree::Glob(_) => "*".to_string(),
            syn::UseTree::Group(_) => "{...}".to_string(),
        }
    }
}

#[atomic_delegates]
impl NoSuperImports {
    pub(crate) fn suggestion(rel_path: &Path, hit: &str) -> (String, String) {}
}

// no test_usage necessary
