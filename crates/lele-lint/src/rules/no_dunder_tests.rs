use crate::Category;
use crate::ExampleFile;
use crate::ItemMarker;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use syn::spanned::Spanned;

pub struct NoDunderTests;

impl NoDunderTests {
    pub const NAME: &'static str = "no_dunder_tests";
    pub const CODE: &'static str = "LELE006";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "No `#[cfg(test)]` inside `__basic__/` containers.",
        why: "Containers hold declarations only; test the function or method that uses the types instead.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod stock;
",
            },
            ExampleFile {
                path: "src/stock/mod.rs",
                source: r#"#[path = "__basic__/mod.rs"]
pub mod basic;

pub use basic::structs::Item;
"#,
            },
            ExampleFile {
                path: "src/stock/__basic__/mod.rs",
                source: r"pub mod structs;
",
            },
            ExampleFile {
                path: "src/stock/__basic__/structs.rs",
                source: r#"pub struct Item {
    pub name: String,
    pub price: u32,
}

#[cfg(test)]
mod tests {
    use super::Item;

    #[test]
    fn test_usage() {
        let item = Item {
            name: "apple".to_string(),
            price: 3,
        };
        assert_eq!(item.price, 3);
    }
}
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod stock;
",
            },
            ExampleFile {
                path: "src/stock/mod.rs",
                source: r#"#[path = "__basic__/mod.rs"]
pub mod basic;

pub use basic::structs::Item;
"#,
            },
            ExampleFile {
                path: "src/stock/__basic__/mod.rs",
                source: r"pub mod structs;
",
            },
            ExampleFile {
                path: "src/stock/__basic__/structs.rs",
                source: r"pub struct Item {
    pub name: String,
    pub price: u32,
}
",
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for NoDunderTests {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoDunderTests)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoDunderTests {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoDunderTests));
    }
    pub(crate) fn cfg_test_line(item: &syn::Item) -> Option<usize> {
        match item {
            syn::Item::Mod(module)
                if module
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_cfg_test()) =>
            {
                Some(module.span().start().line)
            }
            syn::Item::Fn(function)
                if function
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_cfg_test()) =>
            {
                Some(function.span().start().line)
            }
            _ => None,
        }
    }
}

// no test_usage necessary
