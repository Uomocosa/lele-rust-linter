use crate::rules;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct TraitImplCoLocation;

impl TraitImplCoLocation {
    pub const NAME: &'static str = "trait_impl_co_location";
    pub const CODE: &'static str = "LELE042";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every `impl Trait for Type` in `src/` lives in the file that defines `Type` when `Type` is a crate type (last path segment, generics ignored); blanket impls and impls whose `Type` is not defined in the crate live in the file that defines `Trait`.",
        why: "A type's behaviour is readable in one place: a trait impl sits next to the struct or enum it describes, and a crate trait's blanket or foreign-type impls sit next to the trait that owns them — the `<domain>/__traits__/<trait_snake>.rs` file.",
        fix: "Move the impl into the file the message names: `<type>.rs` for a crate type, the trait's own file for a blanket impl or a foreign type. Foreign trait + foreign type pairs cannot happen (orphan rule); an unresolvable type (macro-generated) is skipped.",
        bad: rule_examples!("trait_impl_co_location/bad"),
        good: rule_examples!("trait_impl_co_location/good"),
    };
}

#[rustfmt::skip]
impl Rule for TraitImplCoLocation {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TraitImplCoLocation)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TraitImplCoLocation {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(TraitImplCoLocation));
    }
    pub(crate) fn is_exempt_source(rel_path: &Path) -> bool {
        let file_name = rel_path.file_name().and_then(|name| name.to_str()).unwrap_or("");
        file_name == "build.rs"
            || rel_path
                .components()
                .any(|component| component.as_os_str().to_str() == Some("tests"))
    }
    pub(crate) fn impl_target_message(trait_name: &str, type_name: &str, target: &Path) -> String {
        format!("`impl {trait_name} for {type_name}` must live in `{}`, the file that defines `{type_name}`", target.display())
    }
    pub(crate) fn trait_target_message(trait_name: &str, type_name: &str, target: &Path) -> String {
        format!("`impl {trait_name} for {type_name}` must live in `{}`, the file that defines `{trait_name}`", target.display())
    }
}

#[atomic_delegates]
impl TraitImplCoLocation {
    pub(crate) fn check_hit(
        rel_path: &Path,
        hit: &rules::trait_impl_co_location::ImplHit,
        project: &Project,
        type_definitions: &rules::trait_impl_co_location::DefinitionMap,
        trait_definitions: &rules::trait_impl_co_location::DefinitionMap,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn type_definitions(
        project: &Project,
    ) -> rules::trait_impl_co_location::DefinitionMap {
    }
    pub(crate) fn trait_definitions(
        project: &Project,
    ) -> rules::trait_impl_co_location::DefinitionMap {
    }
}

// no test_usage necessary
