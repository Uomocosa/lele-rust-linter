use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct NoInnerFns;

impl NoInnerFns {
    pub const NAME: &'static str = "no_inner_fns";
    pub const CODE: &'static str = "LELE021";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "No `fn` inside a function body, except a tree walk's inner `fn recursion`.",
        why: "A nested helper hides from the type that owns it; only the recursion's own walk belongs inside its public function (RATIONALE §11).",
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

impl Round {
    pub fn score(&self) -> u32 {
        fn base(hits: u32) -> u32 {
            hits.saturating_mul(10)
        }
        base(self.hits).saturating_sub(self.misses)
    }
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
                source: r#"pub mod tree;

#[path = "../methods/mod.rs"]
pub mod methods;
"#,
            },
            ExampleFile {
                path: "src/tree/mod.rs",
                source: r"mod tree;

pub use tree::Tree;
",
            },
            ExampleFile {
                path: "src/tree/tree.rs",
                source: r"use atomic_delegate_macros::atomic_delegates;

#[derive(Default)]
pub struct Tree {
    pub value: u32,
    pub children: Vec<Tree>,
}

#[atomic_delegates]
impl Tree {
    pub fn total(&self) -> u32 {}
}

#[cfg(test)]
mod tests {
    use crate::tree;

    #[test]
    fn test_usage() {
        assert_eq!(tree::Tree::default().total(), 0);
    }
}
",
            },
            ExampleFile {
                path: "methods/mod.rs",
                source: r"pub mod tree;
",
            },
            ExampleFile {
                path: "methods/tree/mod.rs",
                source: r"mod total;
pub use total::total;
",
            },
            ExampleFile {
                path: "methods/tree/total.rs",
                source: r"use crate::tree;

pub fn total(tree: &tree::Tree) -> u32 {
    fn recursion(tree: &tree::Tree, out: &mut u32) {
        *out = out.saturating_add(tree.value);
        for child in &tree.children {
            recursion(child, out);
        }
    }

    let mut out = 0;
    recursion(tree, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::total;
    use crate::tree::Tree;

    #[test]
    fn test_usage() {
        let leaf = Tree {
            value: 2,
            children: Vec::new(),
        };
        let root = Tree {
            value: 1,
            children: vec![leaf],
        };
        assert_eq!(total(&root), 3);
    }
}
",
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for NoInnerFns {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoInnerFns)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoInnerFns {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoInnerFns));
    }
}

// no test_usage necessary
