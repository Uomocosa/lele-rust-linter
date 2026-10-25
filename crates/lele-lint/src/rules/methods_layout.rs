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

pub struct MethodsLayout;

impl MethodsLayout {
    pub const NAME: &'static str = "methods_layout";
    pub const CODE: &'static str = "LELE030";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "Every `#[atomic_delegates]` shell has a body at `methods/<type>/<method>.rs` with `pub fn <method>`, 2+ statements and its own `test_usage` (or a final `// no test_usage necessary`).",
        why: "`methods/` hides bodies until you need them; the type file stays a short list of what the type can do.",
        fix: "Create `methods/<type>/<method>.rs` with `pub fn <method>` (the receiver becomes the first argument), at least 2 statements and a `test_usage` test, then run `lele-lint --sync-methods <crate>` to update the `methods/` `mod.rs` indexes.",
        bad: rule_examples!("methods_layout/bad"),
        good: rule_examples!("methods_layout/good"),
    };
}

#[rustfmt::skip]
impl Rule for MethodsLayout {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(MethodsLayout)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl MethodsLayout {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(MethodsLayout));
    }
    pub(crate) fn defines_pub_fn(file: &syn::File, name: &str) -> bool {
        file.items.iter().any(|item| {
            matches!(
                item,
                syn::Item::Fn(func)
                    if matches!(func.vis, syn::Visibility::Public(_))
                        && func.sig.ident == name
            )
        })
    }
    pub(crate) fn pub_fn_block<'a>(file: &'a syn::File, name: &str) -> Option<&'a syn::Block> {
        file.items.iter().find_map(|item| match item {
            syn::Item::Fn(func)
                if matches!(func.vis, syn::Visibility::Public(_)) && func.sig.ident == name =>
            {
                Some(&*func.block)
            }
            _ => None,
        })
    }
    pub(crate) fn is_index_file(rel: &Path) -> bool {
        Self::file_stem(rel).is_some_and(|stem| stem == "mod")
    }
    pub(crate) fn parent_dir(rel: &Path) -> Option<String> {
        rel.parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .map(str::to_string)
    }
    pub(crate) fn file_stem(rel: &Path) -> Option<String> {
        rel.file_stem()
            .and_then(|stem| stem.to_str())
            .map(str::to_string)
    }
}

#[atomic_delegates]
impl MethodsLayout {
    pub(crate) fn file_violation(project: &Project, rel: &Path, message: String) -> RuleViolation {}
    pub(crate) fn read_methods_file(project: &Project, rel: &Path) -> Option<String> {}
}

// no test_usage necessary
