use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct NoAllowAttributes;

impl NoAllowAttributes {
    pub const NAME: &'static str = "no_allow_attributes";
    pub const CODE: &'static str = "LELE024";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Style,
        summary: "No `#[allow(...)]`/`#[expect(...)]` unless whitelisted in `lele.toml` with the exact lint, file and a reason.",
        why: "Every silenced lint is a decision the user made on purpose, recorded with its reason.",
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
    #[allow(clippy::needless_pass_by_value)]
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
    #[allow(clippy::needless_pass_by_value)]
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
                path: "lele.toml",
                source: r#"[lele.lint]

[[lele.lint.clippy_allow_whitelist]]
allow = "clippy::needless_pass_by_value"
file = "src/guest.rs"
reason = "example: the signature is fixed by a trait in another crate"
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Checker for NoAllowAttributes {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoAllowAttributes)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoAllowAttributes {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(NoAllowAttributes));
    }
}

// no test_usage necessary
