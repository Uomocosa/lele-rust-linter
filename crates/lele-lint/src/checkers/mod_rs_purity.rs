use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct ModRsPurity;

impl ModRsPurity {
    pub const NAME: &'static str = "mod_rs_purity";
    pub const CODE: &'static str = "LELE023";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "`mod.rs` holds only `mod`/`pub mod` declarations and `pub use` re-exports.",
        why: "`mod.rs` is the folder's table of contents; logic hidden there is logic nobody looks for.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod greeting;
",
            },
            ExampleFile {
                path: "src/greeting/mod.rs",
                source: r#"use derive_more::Deref;

#[derive(Deref)]
pub struct Guest(pub String);

#[rustfmt::skip]
impl Guest {
    pub fn greet(&self) -> String { format!("Hello, {}!", **self) }
}
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod greeting;
",
            },
            ExampleFile {
                path: "src/greeting/mod.rs",
                source: r"mod guest;

pub use guest::Guest;
",
            },
            ExampleFile {
                path: "src/greeting/guest.rs",
                source: r#"use derive_more::Deref;

#[derive(Deref)]
pub struct Guest(pub String);

#[rustfmt::skip]
impl Guest {
    pub fn greet(&self) -> String { format!("Hello, {}!", **self) }
}

#[cfg(test)]
mod tests {
    use super::Guest;

    #[test]
    fn test_usage() {
        assert_eq!(Guest("Ada".to_string()).greet(), "Hello, Ada!");
    }
}
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Checker for ModRsPurity {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ModRsPurity)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ModRsPurity {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(ModRsPurity));
    }
}

// no test_usage necessary
