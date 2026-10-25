use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use rule_examples_macro::rule_examples;

pub struct NoAllowAttributes;

impl NoAllowAttributes {
    pub const NAME: &'static str = "no_allow_attributes";
    pub const CODE: &'static str = "LELE024";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Style,
        summary: "No `#[allow(...)]`/`#[expect(...)]` unless whitelisted in `lele.toml` with the exact lint, file and a reason.",
        why: "Every silenced lint is a decision the user made on purpose, recorded with its reason.",
        bad: rule_examples!("no_allow_attributes/bad"),
        good: rule_examples!("no_allow_attributes/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoAllowAttributes {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoAllowAttributes)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoAllowAttributes {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoAllowAttributes));
    }
    pub(crate) fn lint_paths(attr: &syn::Attribute) -> Vec<String> {
        attr.parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
            .map(|paths| {
                paths
                    .iter()
                    .map(|path| {
                        path.segments
                            .iter()
                            .map(|segment| segment.ident.to_string())
                            .collect::<Vec<_>>()
                            .join("::")
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

// no test_usage necessary
