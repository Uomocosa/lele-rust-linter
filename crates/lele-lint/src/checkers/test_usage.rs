use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct TestUsage;

impl TestUsage {
    pub const NAME: &'static str = "test_usage";
    pub const CODE: &'static str = "LELE009";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "Every non-trivial file has a `#[cfg(test)] mod tests` with a `test_usage` test, or ends with `// no test_usage necessary`.",
        why: "The usage test is the function's documentation: it shows how to call it and proves it works.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod guest;
pub use guest::Guest;
",
            },
            ExampleFile {
                path: "src/guest.rs",
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
                source: r"mod guest;
pub use guest::Guest;
",
            },
            ExampleFile {
                path: "src/guest.rs",
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
impl Checker for TestUsage {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TestUsage)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TestUsage {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(TestUsage));
    }
}

// no test_usage necessary
