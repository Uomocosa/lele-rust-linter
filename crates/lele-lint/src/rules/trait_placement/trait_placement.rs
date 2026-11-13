use crate::rules;
use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct TraitPlacement;

impl TraitPlacement {
    pub const NAME: &'static str = "trait_placement";
    pub const CODE: &'static str = "LELE041";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every trait in `src/` lives in `<domain>/__traits__/<trait_snake>.rs` (crate-root traits in `__traits__/`), one trait per file named after it; a trait never repeats the `Trait` suffix.",
        why: "The `__traits__/` folder is the domain's implementable-role edge: the folder says what the files hold, so each trait keeps its atomic file, named after it, mounted `pub mod traits;` and reached as `domain::traits::<Trait>`.",
        fix: "Move the trait into `<domain>/__traits__/<trait_snake>.rs`, mount the folder with `#[path = \"__traits__/mod.rs\"] pub mod traits;` and refer to the trait as `domain::traits::<Trait>` — never re-export it (`pub use traits::X;` is a LELE038 error); rename a `…Trait` trait to its bare name — the folder already says it is a trait.",
        bad: rule_examples!("trait_placement/bad"),
        good: rule_examples!("trait_placement/good"),
    };
}

#[rustfmt::skip]
impl Rule for TraitPlacement {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TraitPlacement, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TraitPlacement {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(TraitPlacement));
    }
    pub(crate) fn misplaced_message(trait_name: &str, target: &Path) -> String {
        format!("trait `{trait_name}` must live in `{}`", target.display())
    }
}

impl TraitPlacement {
    pub(crate) fn is_exempt_source(rel_path: &Path) -> bool {
        let file_name = rel_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        file_name == "build.rs"
            || rel_path
                .components()
                .any(|component| component.as_os_str().to_str() == Some("tests"))
    }
    pub(crate) fn suffix_message(trait_name: &str, target: &Path) -> String {
        let bare = trait_name.strip_suffix("Trait").unwrap_or(trait_name);
        format!("trait `{trait_name}` must drop the `Trait` suffix — declare it as `{bare}` so it lives in `{}`", target.display())
    }
}

#[atomic_delegates]
impl TraitPlacement {
    pub(crate) fn check_hit(
        rel_path: &Path,
        hit: &rules::trait_placement::TraitHit,
        project: &Project,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn trait_target_path(rel_path: &Path, trait_name: &str) -> std::path::PathBuf {}
}

// no test_usage necessary
