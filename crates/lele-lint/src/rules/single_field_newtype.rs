use crate::Category;
use crate::ExampleFile;
use crate::ItemMarker;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct SingleFieldNewtype;

impl SingleFieldNewtype {
    pub const NAME: &'static str = "single_field_newtype";
    pub const CODE: &'static str = "LELE022";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "A struct with one field is a tuple newtype deriving `Deref`; two or more fields are named.",
        why: "Field count decides the shape, so every single-value wrapper reads the same: `*meters`, never `meters.value`.",
        bad: rule_examples!("single_field_newtype/bad"),
        good: rule_examples!("single_field_newtype/good"),
    };
}

#[rustfmt::skip]
impl Rule for SingleFieldNewtype {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(SingleFieldNewtype)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl SingleFieldNewtype {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(SingleFieldNewtype));
    }
    pub(crate) fn has_deref_derive(struct_def: &syn::ItemStruct) -> bool {
        struct_def
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).derives_any_of(&["Deref"]))
    }
}

#[atomic_delegates]
impl SingleFieldNewtype {
    pub(crate) fn check_struct(
        struct_def: &syn::ItemStruct,
        file_path: &Path,
    ) -> Vec<RuleViolation> {
    }
}

// no test_usage necessary
