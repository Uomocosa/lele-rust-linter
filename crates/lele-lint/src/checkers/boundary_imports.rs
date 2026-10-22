use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct BoundaryImports;

impl BoundaryImports {
    pub const NAME: &'static str = "boundary_imports";
    pub const CODE: &'static str = "LELE033";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "A `[[lele.boundary]]` folder may not use any path from its `cannot_use` list.",
        why: "A boundary marks a pure core; forbidden imports are how I/O and frameworks leak in.",
        bad: &[
            ExampleFile {
                path: "lele.toml",
                source: r#"[[lele.boundary]]
name = "session is pure"
why = "no io"
folders = ["src/session"]
cannot_use = ["std::time"]
require = "honest"
"#,
            },
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod session;
",
            },
            ExampleFile {
                path: "src/session/mod.rs",
                source: r"mod session;
pub use session::Session;
",
            },
            ExampleFile {
                path: "src/session/session.rs",
                source: r"use derive_more::Deref;

#[derive(Deref)]
pub struct Session(pub u64);

#[rustfmt::skip]
impl Session {
    pub fn expiry(&self) -> u64 { std::time::SystemTime::now().elapsed().map_or(**self, |_| self.saturating_sub(1)) }
}
",
            },
        ],
        good: &[
            ExampleFile {
                path: "lele.toml",
                source: r#"[[lele.boundary]]
name = "session is pure"
why = "no io"
folders = ["src/session"]
cannot_use = ["std::time"]
require = "honest"
"#,
            },
            ExampleFile {
                path: "src/lib.rs",
                source: r"pub mod session;
",
            },
            ExampleFile {
                path: "src/session/mod.rs",
                source: r"mod session;
pub use session::Session;
",
            },
            ExampleFile {
                path: "src/session/session.rs",
                source: r"use derive_more::Deref;

#[derive(Deref)]
pub struct Session(pub u64);

#[rustfmt::skip]
impl Session {
    pub fn expiry(&self) -> u64 { self.saturating_sub(1) }
}

#[cfg(test)]
mod tests {
    use super::Session;

    #[test]
    fn test_usage() {
        assert_eq!(Session(3).expiry(), 2);
    }
}
",
            },
        ],
    };
}

#[rustfmt::skip]
impl Checker for BoundaryImports {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BoundaryImports)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BoundaryImports {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(BoundaryImports));
    }
}

// no test_usage necessary
