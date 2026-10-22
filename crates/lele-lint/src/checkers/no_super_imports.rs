use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct NoSuperImports;

impl NoSuperImports {
    pub const NAME: &'static str = "no_super_imports";
    pub const CODE: &'static str = "LELE013";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "`super::` is allowed only inside `#[cfg(test)]`; production code imports the domain (`use crate::stock;`).",
        why: "`super::` paths break when files move; domain paths read the same from every file.",
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
mod shelf;
pub use shelf::Shelf;
"#,
            },
            ExampleFile {
                path: "src/stock/shelf.rs",
                source: r"use super::Item;

pub struct Shelf {
    pub label: String,
    pub items: Vec<Item>,
}

#[rustfmt::skip]
impl Shelf {
    pub fn total_price(&self) -> u32 { self.items.iter().map(|item| item.price).sum() }
}
",
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
mod shelf;
pub use shelf::Shelf;
"#,
            },
            ExampleFile {
                path: "src/stock/shelf.rs",
                source: r#"use crate::stock;

pub struct Shelf {
    pub label: String,
    pub items: Vec<stock::Item>,
}

#[rustfmt::skip]
impl Shelf {
    pub fn total_price(&self) -> u32 { self.items.iter().map(|item| item.price).sum() }
}

#[cfg(test)]
mod tests {
    use super::Shelf;
    use crate::stock::Item;

    #[test]
    fn test_usage() {
        let shelf = Shelf {
            label: "fruit".to_string(),
            items: vec![Item {
                name: "apple".to_string(),
                price: 3,
            }],
        };
        assert_eq!(shelf.total_price(), 3);
    }
}
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
impl Checker for NoSuperImports {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoSuperImports)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoSuperImports {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(NoSuperImports));
    }
}

// no test_usage necessary
