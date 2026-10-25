use crate::Category;
use crate::DunderPath;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::ffi::OsStr;

pub struct DunderModuleName;

impl DunderModuleName {
    pub const NAME: &'static str = "dunder_module_name";
    pub const CODE: &'static str = "LELE038";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "A dunder folder or file (`__basic__/`) is declared `#[path = \"__basic__/mod.rs\"] pub mod basic;` and named `basic` everywhere in code.",
        why: "The dunders mark the folder on disk (sorted first, not a domain); in code they are noise, so the module carries the plain name.",
        bad: rule_examples!("dunder_module_name/bad"),
        good: rule_examples!("dunder_module_name/good"),
    };
}

#[rustfmt::skip]
impl Rule for DunderModuleName {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(DunderModuleName)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl DunderModuleName {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(DunderModuleName));
    }
    pub(crate) fn dunder_named<'a>(dunders: &'a [DunderPath], ident: &str) -> Option<&'a DunderPath> {
        dunders
            .iter()
            .find(|dunder| dunder.file_stem() == Some(OsStr::new(ident)))
    }
}

#[atomic_delegates]
impl DunderModuleName {
    pub(crate) fn dunder_in_path<'a>(
        dunders: &'a [DunderPath],
        target: &str,
    ) -> Option<&'a DunderPath> {
    }
    pub(crate) fn path_attribute(attrs: &[syn::Attribute]) -> Option<String> {}
    pub(crate) fn default_path(dunder: &DunderPath) -> String {}
}

// no test_usage necessary
