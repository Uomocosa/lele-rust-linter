use crate::traits::Rule;
use crate::Category;
use crate::CollectionWrapperEntry;
use crate::ExampleFile;
use crate::ItemMarker;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct NoCollectionNewtype;

impl NoCollectionNewtype {
    pub const NAME: &'static str = "no_collection_newtype";
    pub const CODE: &'static str = "LELE026";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "No newtype around a collection (`Rooms(Vec<String>)`); wrap the element (`Room(String)`) and use `Vec<Room>`. A wrapper that guards an invariant of the collection is allowed when listed in `[[lele.lint.collection_wrappers]]` with a reason, and it has a private field, `Deref` without `DerefMut`, and an impl. A wrapper over an unsigned-integer collection (`Bytes(Vec<u8>)`) is exempt: neither `u8` nor `Vec<u8>` says what the bytes are.",
        why: "The element is the concept; a collection wrapper hides the standard collection API behind a new name.",
        fix: "Delete the collection wrapper; wrap the element instead (`Room(String)`) and hold a plain `Vec<Room>`. Bevy Resource types are exempt: Bevy needs a named type for a resource. Keep a wrapper only when the collection has a rule every holder must respect (size cap, ordering, uniqueness) — `Deck(Vec<Card>)` with at most 52 cards is one, `Rooms(Vec<Room>)` with nothing to enforce is not. Then ask the user to list it in `[[lele.lint.collection_wrappers]]` (the entry is the user's decision, never yours), give it a private field, `#[derive(Deref)]` without `DerefMut`, and check the invariant in `TryFrom` or its methods; inside them reach the collection with `let Self(cards) = self;`.",
        bad: rule_examples!("no_collection_newtype/bad"),
        good: rule_examples!("no_collection_newtype/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoCollectionNewtype {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoCollectionNewtype)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoCollectionNewtype {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoCollectionNewtype));
    }
    pub(crate) fn has_unsigned_element(ty: &syn::Type) -> bool {
        Self::element_type(ty).is_some_and(Self::is_unsigned_int)
    }
    pub(crate) fn singular_name(name: &str) -> String {
        if let Some(stripped) = name.strip_suffix("ies") {
            format!("{stripped}y")
        } else if let Some(stripped) = name.strip_suffix("ses") {
            stripped.to_string()
        } else if let Some(stripped) = name.strip_suffix("s") {
            stripped.to_string()
        } else {
            format!("{name}Item")
        }
    }
    pub(crate) fn derives_deref_mut(struct_def: &syn::ItemStruct) -> bool { struct_def.attrs.iter().any(|attr| ItemMarker(attr).derives_any_of(&["DerefMut"])) }
    pub(crate) fn has_public_field(struct_def: &syn::ItemStruct) -> bool { struct_def.fields.iter().any(|field| !matches!(field.vis, syn::Visibility::Inherited)) }
    pub(crate) fn impl_self_is(imp: &syn::ItemImpl, name: &str) -> bool { matches!(&*imp.self_ty, syn::Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == name)) }
    pub(crate) fn has_invariant_impl(items: &[syn::Item], name: &str) -> bool { items.iter().any(|item| matches!(item, syn::Item::Impl(imp) if Self::is_invariant_impl(imp, name))) }
    pub(crate) fn listed_entry(project: &Project, struct_def: &syn::ItemStruct, file_path: &Path) -> Option<CollectionWrapperEntry> { project.collection_wrappers.iter().find(|entry| struct_def.ident == entry.type_name && project.root.join(&entry.file) == file_path).cloned() }
}

#[atomic_delegates]
impl NoCollectionNewtype {
    pub(crate) fn check_struct(
        struct_def: &syn::ItemStruct,
        file_path: &Path,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_struct_for_project(
        struct_def: &syn::ItemStruct,
        file_path: &Path,
        project: &Project,
        items: &[syn::Item],
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_listed_wrapper(
        struct_def: &syn::ItemStruct,
        file_path: &Path,
        items: &[syn::Item],
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn is_invariant_impl(imp: &syn::ItemImpl, name: &str) -> bool {}
    pub(crate) fn wraps_collection(struct_def: &syn::ItemStruct) -> bool {}
    pub(crate) fn defines_listed_wrapper(
        project: &Project,
        entry: &CollectionWrapperEntry,
    ) -> bool {
    }
    pub(crate) fn entry_violations(project: &Project) -> Vec<RuleViolation> {}
    pub(crate) fn collection_name(ty: &syn::Type) -> Option<&'static str> {}
    pub(crate) fn derives_resource(struct_def: &syn::ItemStruct) -> bool {}
    pub(crate) fn boxed_slice_element(segment: &syn::PathSegment) -> Option<&syn::Type> {}
    pub(crate) fn element_type(ty: &syn::Type) -> Option<&syn::Type> {}
    pub(crate) fn first_generic_type(segment: &syn::PathSegment) -> Option<&syn::Type> {}
    pub(crate) fn is_unsigned_int(ty: &syn::Type) -> bool {}
}

// no test_usage necessary
