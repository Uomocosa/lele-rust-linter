use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct TestInline;

impl TestInline {
    pub const NAME: &'static str = "test_inline";
    pub const CODE: &'static str = "LELE007";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "Unit tests live in the same file as the code (no `tests/` folders under `src/`).",
        why: "The test sits next to what it tests, so reading one means reading the other.",
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
            ExampleFile {
                path: "src/tests/greet_more.rs",
                source: r#"#[test]
fn greets_empty_name() {
    assert_eq!(crate::Guest(String::new()).greet(), "Hello, !");
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
impl Checker for TestInline {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TestInline)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TestInline {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(TestInline));
    }
}

// no test_usage necessary
