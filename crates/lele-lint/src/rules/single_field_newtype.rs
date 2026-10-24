use crate::Category;
use crate::ExampleFile;
use crate::ItemMarker;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use std::path::Path;

pub struct SingleFieldNewtype;

impl SingleFieldNewtype {
    pub const NAME: &'static str = "single_field_newtype";
    pub const CODE: &'static str = "LELE022";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "A struct with one field is a tuple newtype deriving `Deref`; two or more fields are named.",
        why: "Field count decides the shape, so every single-value wrapper reads the same: `*meters`, never `meters.value`.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod track;
",
            },
            ExampleFile {
                path: "src/track/mod.rs",
                source: r#"#[path = "__basic__/mod.rs"]
pub mod basic;

pub use basic::structs::Meters;
"#,
            },
            ExampleFile {
                path: "src/track/__basic__/mod.rs",
                source: r"pub mod structs;
",
            },
            ExampleFile {
                path: "src/track/__basic__/structs.rs",
                source: r"pub struct Meters {
    pub value: u32,
}
",
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod track;
",
            },
            ExampleFile {
                path: "src/track/mod.rs",
                source: r#"#[path = "__basic__/mod.rs"]
pub mod basic;

pub use basic::newtypes::Meters;
"#,
            },
            ExampleFile {
                path: "src/track/__basic__/mod.rs",
                source: r"pub mod newtypes;
",
            },
            ExampleFile {
                path: "src/track/__basic__/newtypes.rs",
                source: r"use derive_more::Deref;

#[derive(Deref)]
pub struct Meters(pub u32);
",
            },
        ],
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
