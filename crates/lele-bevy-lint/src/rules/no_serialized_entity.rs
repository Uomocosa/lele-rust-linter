use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct NoSerializedEntity;

impl NoSerializedEntity {
    pub const NAME: &'static str = "no_serialized_entity";
    pub const CODE: &'static str = "LELEBEVY009";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "A struct or enum that derives `Serialize` or `Deserialize` must not contain `Entity` in any field type, including nested generics (`Vec<Entity>`, `Option<Entity>`, tuples, arrays).",
        why: "An `Entity` id is only meaningful inside one running `World`; serialized it becomes a dangling number on load or on another peer.",
        fix: "Send the stable domain id instead (e.g. a `PeerId` / `RoomName`) and map it to an `Entity` at runtime.",
        bad: rule_examples!("no_serialized_entity/bad"),
        good: rule_examples!("no_serialized_entity/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoSerializedEntity {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoSerializedEntity)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoSerializedEntity {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

impl NoSerializedEntity {
    pub(crate) fn struct_has_entity(struct_def: &syn::ItemStruct) -> bool {
        match &struct_def.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .any(|field| Self::contains_entity(&field.ty)),
            syn::Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .any(|field| Self::contains_entity(&field.ty)),
            syn::Fields::Unit => false,
        }
    }
}

#[atomic_delegates]
impl NoSerializedEntity {
    pub(crate) fn is_serialized(attrs: &[syn::Attribute]) -> bool {}
    pub(crate) fn contains_entity(ty: &syn::Type) -> bool {}
    pub(crate) fn enum_has_entity(item_enum: &syn::ItemEnum) -> bool {}
}

// no test_usage necessary
