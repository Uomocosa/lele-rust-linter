use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct NoCratePaths;

impl NoCratePaths {
    pub const NAME: &'static str = "no_crate_paths";
    pub const CODE: &'static str = "LELE025";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "`crate::` appears only in `use` items (outside `lib.rs`/`main.rs`).",
        why: "All cross-domain dependencies of a file are then visible in its `use` lines at the top.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod shop;
pub mod stock;
",
            },
            ExampleFile {
                path: "src/shop/mod.rs",
                source: r"mod cart;

pub use cart::Cart;
",
            },
            ExampleFile {
                path: "src/shop/cart.rs",
                source: r#"pub struct Cart {
    pub owner: String,
    pub items: Vec<crate::stock::Item>,
}

#[rustfmt::skip]
impl Cart {
    pub fn total(&self) -> u32 { self.items.iter().map(|item| item.price).sum() }
}

#[cfg(test)]
mod tests {
    use crate::shop;
    use crate::stock;

    #[test]
    fn test_usage() {
        let cart = shop::Cart {
            owner: "ada".to_string(),
            items: vec![stock::Item {
                name: "apple".to_string(),
                price: 3,
            }],
        };
        assert_eq!(cart.total(), 3);
    }
}
"#,
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
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod shop;
pub mod stock;
",
            },
            ExampleFile {
                path: "src/shop/mod.rs",
                source: r"mod cart;

pub use cart::Cart;
",
            },
            ExampleFile {
                path: "src/shop/cart.rs",
                source: r#"use crate::stock;

pub struct Cart {
    pub owner: String,
    pub items: Vec<stock::Item>,
}

#[rustfmt::skip]
impl Cart {
    pub fn total(&self) -> u32 { self.items.iter().map(|item| item.price).sum() }
}

#[cfg(test)]
mod tests {
    use crate::shop;
    use crate::stock;

    #[test]
    fn test_usage() {
        let cart = shop::Cart {
            owner: "ada".to_string(),
            items: vec![stock::Item {
                name: "apple".to_string(),
                price: 3,
            }],
        };
        assert_eq!(cart.total(), 3);
    }
}
"#,
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
impl Checker for NoCratePaths {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoCratePaths)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoCratePaths {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(NoCratePaths));
    }
}

// no test_usage necessary
