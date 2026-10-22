use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct AtomicFile;

impl AtomicFile {
    pub const NAME: &'static str = "atomic_file";
    pub const CODE: &'static str = "LELE001";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "One public item per file, and the file is named after it (`guest.rs` holds `pub struct Guest`).",
        why: "A file name tells you exactly what is inside; finding code means finding a file, and every unit gets its own test.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod greeting;
pub use greeting::{farewell, greet};
",
            },
            ExampleFile {
                path: "src/greeting.rs",
                source: r#"pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn farewell(name: &str) -> String {
    format!("Bye, {name}!")
}
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod guest;
mod host;
pub use guest::Guest;
pub use host::Host;
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
                path: "src/host.rs",
                source: r#"use derive_more::Deref;

#[derive(Deref)]
pub struct Host(pub String);

#[rustfmt::skip]
impl Host {
    pub fn farewell(&self) -> String { format!("Bye, {}!", **self) }
}

#[cfg(test)]
mod tests {
    use super::Host;

    #[test]
    fn test_usage() {
        assert_eq!(Host("Ada".to_string()).farewell(), "Bye, Ada!");
    }
}
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Checker for AtomicFile {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(AtomicFile)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl AtomicFile {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(AtomicFile));
    }
}

// no test_usage necessary
