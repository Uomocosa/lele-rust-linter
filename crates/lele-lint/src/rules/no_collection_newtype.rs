use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use std::path::Path;

pub struct NoCollectionNewtype;

impl NoCollectionNewtype {
    pub const NAME: &'static str = "no_collection_newtype";
    pub const CODE: &'static str = "LELE026";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "No newtype around a collection (`Rooms(Vec<String>)`); wrap the element (`Room(String)`) and use `Vec<Room>`.",
        why: "The element is the concept; a collection wrapper hides the standard collection API behind a new name.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod lobby;
",
            },
            ExampleFile {
                path: "src/lobby/mod.rs",
                source: r#"#[path = "__basic__/mod.rs"]
pub mod basic;

pub use basic::newtypes::Rooms;
"#,
            },
            ExampleFile {
                path: "src/lobby/__basic__/mod.rs",
                source: r"pub mod newtypes;
",
            },
            ExampleFile {
                path: "src/lobby/__basic__/newtypes.rs",
                source: r"use derive_more::Deref;

#[derive(Deref)]
pub struct Rooms(pub Vec<String>);
",
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod lobby;
",
            },
            ExampleFile {
                path: "src/lobby/mod.rs",
                source: r#"#[path = "__basic__/mod.rs"]
pub mod basic;

pub use basic::newtypes::Room;
"#,
            },
            ExampleFile {
                path: "src/lobby/__basic__/mod.rs",
                source: r"pub mod newtypes;
",
            },
            ExampleFile {
                path: "src/lobby/__basic__/newtypes.rs",
                source: r"use derive_more::Deref;

#[derive(Deref)]
pub struct Room(pub String);
",
            },
        ],
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
}

#[atomic_delegates]
impl NoCollectionNewtype {
    pub(crate) fn check_struct(
        struct_def: &syn::ItemStruct,
        file_path: &Path,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn collection_name(ty: &syn::Type) -> Option<&'static str> {}
    pub(crate) fn boxed_slice_element(segment: &syn::PathSegment) -> Option<&syn::Type> {}
    pub(crate) fn element_type(ty: &syn::Type) -> Option<&syn::Type> {}
    pub(crate) fn first_generic_type(segment: &syn::PathSegment) -> Option<&syn::Type> {}
    pub(crate) fn is_unsigned_int(ty: &syn::Type) -> bool {}
}

// no test_usage necessary
