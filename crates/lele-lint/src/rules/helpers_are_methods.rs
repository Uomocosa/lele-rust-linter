use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct HelpersAreMethods;

impl HelpersAreMethods {
    pub const NAME: &'static str = "helpers_are_methods";
    pub const CODE: &'static str = "LELE008";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "No private `fn` in `src/`: a helper is a `pub(crate)` method or associated fn of the type it serves, never a private free fn or a private method. `fn main` in `main.rs` and test code are exempt.",
        why: "`src/` lists types and what each can do; `pub` vs `pub(crate)` on the type's own methods tells API from internal machinery, so no helper hides outside the type.",
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
    pub fn score(&self) -> u32 { base(self.hits).saturating_sub(self.misses) }
}

fn base(hits: u32) -> u32 {
    hits.saturating_mul(10)
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
                source: r"pub struct Round {
    pub hits: u32,
    pub misses: u32,
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
        assert_eq!(Round { hits: 1, misses: 0 }.score(), 10);
    }
}
",
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for HelpersAreMethods {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(HelpersAreMethods)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl HelpersAreMethods {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(HelpersAreMethods));
    }
}

// no test_usage necessary
