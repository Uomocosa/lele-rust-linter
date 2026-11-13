use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;

pub struct BevyExport;

impl BevyExport {
    pub const NAME: &'static str = "bevy_export";
    pub const CODE: &'static str = "LELEBEVY001";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "A domain never re-exports its `bevy_systems` items (any file, `lib.rs` included); callers write `domain::bevy_systems::system`.",
        why: "Consumers reach systems as `domain::bevy_systems::system`, so the folder is always visible in the path. Detection is shared with lele-lint's `dunder_module_name` (which reports the `traits` mounts), so each re-export is reported once.",
        fix: "Delete every `pub use`/`pub(crate) use` whose path goes through `bevy_systems` and make callers write `<domain>::bevy_systems::<system>`; keep the `pub use` inside `__bevy_systems__/mod.rs`.",
        bad: rule_examples!("bevy_export/bad"),
        good: rule_examples!("bevy_export/good"),
    };
}

#[rustfmt::skip]
impl Rule for BevyExport {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyExport, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyExport {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
