use crate::Category;
use crate::ExampleFile;
use crate::Identifier;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

pub struct MethodFileCoLocation;

impl MethodFileCoLocation {
    pub const NAME: &'static str = "method_file_co_location";
    pub const CODE: &'static str = "LELE003";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "A `<type>_<method>.rs` file must sit in the same folder as `<type>.rs`.",
        why: "A method file that drifts away from its type is impossible to find from the type.",
        fix: "Move `<type>_<method>.rs` into the folder that holds `<type>.rs`; if it is not a method of that type, rename it after its own item.",
        bad: rule_examples!("method_file_co_location/bad"),
        good: rule_examples!("method_file_co_location/good"),
    };
}

#[rustfmt::skip]
impl Rule for MethodFileCoLocation {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(MethodFileCoLocation)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl MethodFileCoLocation {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(MethodFileCoLocation));
    }
    pub(crate) fn pub_fn_names(file: &syn::File) -> Vec<String> {
        file.items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Fn(f) if matches!(f.vis, syn::Visibility::Public(_)) => {
                    Some(f.sig.ident.to_string())
                }
                _ => None,
            })
            .collect()
    }
    pub(crate) fn find_parent_type(
        stem: &str,
        known_stems: &HashSet<String>,
        type_dirs: &HashMap<String, Vec<PathBuf>>,
    ) -> Option<Vec<PathBuf>> {
        Identifier(stem)
            .longest_type_prefix(known_stems)
            .and_then(|prefix| type_dirs.get(prefix).cloned())
    }
}

#[atomic_delegates]
impl MethodFileCoLocation {
    pub(crate) fn build_type_map(project: &Project) -> HashMap<String, Vec<PathBuf>> {}
}

// no test_usage necessary
