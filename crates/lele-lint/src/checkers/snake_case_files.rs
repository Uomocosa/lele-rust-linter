use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct SnakeCaseFiles;

impl SnakeCaseFiles {
    pub const NAME: &'static str = "snake_case_files";
    pub const CODE: &'static str = "LELE002";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "File and folder names are snake_case.",
        why: "Module names are derived from file names; one casing keeps paths predictable.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r#"#[path = "Guest.rs"]
mod guest;
pub use guest::Guest;
"#,
            },
            ExampleFile {
                path: "src/Guest.rs",
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
impl Checker for SnakeCaseFiles {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(SnakeCaseFiles)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl SnakeCaseFiles {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(SnakeCaseFiles));
    }
}

// no test_usage necessary
