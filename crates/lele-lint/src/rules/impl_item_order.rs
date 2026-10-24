use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct ImplItemOrder;

impl ImplItemOrder {
    pub const NAME: &'static str = "impl_item_order";
    pub const CODE: &'static str = "LELE027";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "A type's impls in `src/` follow one order: `impl Default`, constructors (receiver-less fns returning `Self`, `impl From`/`TryFrom`/`FromStr`), other trait impls, `pub` fns, then `pub(crate)` fns.",
        why: "Every type file reads the same way: how a value is made, what it is, its API, then its internal machinery.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod round;
pub use round::Round;
",
            },
            ExampleFile {
                path: "src/round.rs",
                source: r"pub struct Round {
    pub hits: u32,
    pub misses: u32,
}

#[rustfmt::skip]
impl Round {
    pub(crate) fn base(hits: u32) -> u32 { hits.saturating_mul(10) }
    pub fn score(&self) -> u32 { Self::base(self.hits).saturating_sub(self.misses) }
}

#[cfg(test)]
mod tests {
    use super::Round;

    #[test]
    fn test_usage() {
        assert_eq!(Round { hits: 1, misses: 0 }.score(), 10);
    }
}
",
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod round;
pub use round::Round;
",
            },
            ExampleFile {
                path: "src/round.rs",
                source: r#"use std::fmt;

pub struct Round {
    pub hits: u32,
    pub misses: u32,
}

impl Default for Round {
    fn default() -> Self {
        Self { hits: 0, misses: 0 }
    }
}

#[rustfmt::skip]
impl From<u32> for Round {
    fn from(hits: u32) -> Self { Self { hits, misses: 0 } }
}

#[rustfmt::skip]
impl fmt::Display for Round {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { write!(formatter, "{}/{}", self.hits, self.misses) }
}

#[rustfmt::skip]
impl Round {
    pub fn score(&self) -> u32 { Self::base(self.hits).saturating_sub(self.misses) }
    pub(crate) fn base(hits: u32) -> u32 { hits.saturating_mul(10) }
}

#[cfg(test)]
mod tests {
    use super::Round;

    #[test]
    fn test_usage() {
        assert_eq!(Round::from(1).score(), 10);
        assert_eq!(Round::default().to_string(), "0/0");
    }
}
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for ImplItemOrder {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ImplItemOrder)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ImplItemOrder {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ImplItemOrder));
    }
}

// no test_usage necessary
