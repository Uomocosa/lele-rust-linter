use std::path::Path;

use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;

pub struct AtomicDelegates;

impl AtomicDelegates {
    pub const NAME: &'static str = "atomic_delegates";
    pub const CODE: &'static str = "LELE017";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "Each hand-written method of an impl block (inherent or trait; `impl Default` and methods listed in `[[lele.lint.inline_trait_methods]]` are exempt) has one of four kinds, and each kind has its home: a one-liner (one statement that rustfmt prints on one line) sits in a `#[rustfmt::skip]` block; a single simple `match`/`if`/loop, or a method of 2 or 3 statements, sits in a plain `impl` block; anything else (a statement rustfmt breaks over several lines, a `match`/`if`/loop whose branches do more than one thing, more than 3 statements) lives in `__methods__/`. A crate listed in `[[lele.lint.no_delegates]]` with a reason may keep those anything-else methods in a plain `impl` block instead.",
        why: "rustfmt is the judge of layout, so the linter asks rustfmt rather than reading the author's line breaks: a long chain typed on one line under `#[rustfmt::skip]` is not a one-liner. Plain blocks keep rustfmt formatting for multi-line methods; longer bodies belong in `__methods__/<type>/<method>.rs`.",
        fix: "Write one-liners on one line in a `#[rustfmt::skip]` impl block. Write a single simple `match`/`if`/loop, or a method of 2 or 3 statements, in a plain `impl` block so rustfmt formats it. Move anything else (a statement rustfmt prints over several lines, a `match`/`if`/loop whose branches do more than one thing, more than 3 statements) to `__methods__/<type>/<method>.rs` (a trait method's file is named by its trait-prefixed stem, e.g. `rule_check.rs`) behind an empty shell in the type's `#[atomic_delegates]` block. Exemption: a crate listed in `[[lele.lint.no_delegates]]` (the entry is the user's decision, never yours; it needs a reason and an existing crate path) may keep them in a plain `impl` block; justified only when the crate cannot depend on `atomic-delegate-macros`, which is the case for `lele-snake-case`.",
        bad: rule_examples!("atomic_delegates/bad"),
        good: rule_examples!("atomic_delegates/good"),
    };
}

#[rustfmt::skip]
impl Rule for AtomicDelegates {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(AtomicDelegates, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl AtomicDelegates {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(AtomicDelegates));
    }
}

#[atomic_delegates]
impl AtomicDelegates {
    pub(crate) fn has_any_method(impl_block: &syn::ItemImpl) -> bool {}
    pub(crate) fn names_str(idents: &[&syn::Ident]) -> String {}
    pub(crate) fn is_one_line_body(method: &syn::ImplItemFn) -> bool {}
    pub(crate) fn is_no_delegates_crate(project: &Project) -> bool {}
    pub(crate) fn no_delegates_violations(project: &Project) -> Vec<RuleViolation> {}
    pub(crate) fn plain_block_violations(
        project: &Project,
        rel_path: &Path,
        impl_block: &syn::ItemImpl,
    ) -> Vec<RuleViolation> {
    }
}

// no test_usage necessary
