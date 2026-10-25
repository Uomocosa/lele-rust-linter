use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use crate::Visibility;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use lele_snake_case::ToLeleSnakeCase;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct NoStutteredType;

impl NoStutteredType {
    pub const NAME: &'static str = "no_stuttered_type";
    pub const CODE: &'static str = "LELE012";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "A type does not repeat its folder's name (`freenet::Client`, not `freenet::FreenetClient`).",
        why: "Code always says `freenet::Client` through the domain prefix, so the prefix in the type name is said twice.",
        bad: rule_examples!("no_stuttered_type/bad"),
        good: rule_examples!("no_stuttered_type/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoStutteredType {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoStutteredType)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoStutteredType {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoStutteredType));
    }
    pub(crate) fn parent_dir_name(rel_path: &Path) -> Option<String> {
        rel_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(str::to_string)
    }
    pub(crate) fn primary_exposed_type(file: &syn::File, stem: &str) -> Option<String> {
        file.items.iter().find_map(|item| {
            let (ident, vis) = match item {
                syn::Item::Struct(s) => (&s.ident, &s.vis),
                syn::Item::Enum(e) => (&e.ident, &e.vis),
                _ => return None,
            };
            if !Visibility(vis).is_exposed() {
                return None;
            }
            let name = ident.to_string();
            if name.to_snake_case() == stem {
                Some(name)
            } else {
                None
            }
        })
    }
}

#[atomic_delegates]
impl NoStutteredType {
    pub(crate) fn strip_dir_prefix(snake: &str, dir: &str) -> Option<String> {}
}

// no test_usage necessary
