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

pub struct TraitImplCoLocation;

impl TraitImplCoLocation {
    pub const NAME: &'static str = "trait_impl_co_location";
    pub const CODE: &'static str = "LELE042";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Every `impl Trait for Type` in `src/` lives in the file that defines `Type` when `Type` is a crate type (last path segment, generics ignored, references and parens unwrapped); blanket impls and impls whose `Type` is not defined in the crate live in the file that defines `Trait`; when several files define a type or trait of that name, the impl is only clean in a file that defines it — otherwise the impl's own qualified path or a matching `use` import in its file selects the defining file (relative `super::`/`self::` qualifiers resolve against the impl file's own module, and dunder-mounted modules match by their code name, `stock::traits` → `stock/__traits__/`), and an unresolved ambiguity lists every defining file.",
        why: "A type's behaviour is readable in one place: a trait impl sits next to the struct or enum it describes, and a crate trait's blanket or foreign-type impls sit next to the trait that owns them — the `<domain>/__traits__/<trait_snake>.rs` file.",
        fix: "Move the impl into the file the message names: `<type>.rs` for a crate type, the trait's own file for a blanket impl or a foreign type. When several files define a type of the same name, an impl is clean only in a file that defines it; the impl's own qualified path or a matching `use` import in its file selects the defining file — a relative (`super::`/`self::`) import is resolved against the impl file's own module and a `…::traits`/`…::basic` import matches the `__traits__/`/`__basic__/` files of that module — and an unresolved ambiguity lists every defining file. The trait side resolves the same way: when several `__traits__/` files define a trait of that name, the impl's own trait path (`impl crate::a::Foo for T`) or a `use` import of the trait name selects the defining file, and an unresolved ambiguity lists every defining `__traits__/` file. A type in `__basic__/` keeps no impls — move it to its own `<type>.rs` first and put the impl there. Foreign trait + foreign type pairs cannot happen (orphan rule); an unresolvable type (macro-generated) is skipped.",
        bad: rule_examples!("trait_impl_co_location/bad"),
        good: rule_examples!("trait_impl_co_location/good"),
    };
}

#[rustfmt::skip]
impl Rule for TraitImplCoLocation {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TraitImplCoLocation, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TraitImplCoLocation {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(TraitImplCoLocation));
    }
}

impl TraitImplCoLocation {
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
    pub(crate) fn ambiguous_target_message(
        name: &str,
        candidates: &std::collections::BTreeSet<std::path::PathBuf>,
    ) -> String {
        let files = candidates
            .iter()
            .map(|candidate| candidate.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("`{name}` is defined in several files ({files}): move the impl into the one that defines this `{name}`")
    }
}

#[atomic_delegates]
impl TraitImplCoLocation {
    pub(crate) fn container_target_message(
        trait_name: &str,
        type_name: &str,
        target: &Path,
        graduate: &Path,
    ) -> String {
    }
    pub(crate) fn impl_target_message(trait_name: &str, type_name: &str, target: &Path) -> String {}
    pub(crate) fn trait_target_message(trait_name: &str, type_name: &str, target: &Path) -> String {
    }
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
    pub(crate) fn self_type_name(item: &syn::ItemImpl) -> Option<String> {}
    pub(crate) fn type_graduate_target(target: &Path, type_name: &str) -> std::path::PathBuf {}
    pub(crate) fn impl_type_path(item: &syn::ItemImpl) -> Option<Vec<String>> {}
    pub(crate) fn impl_trait_path(item: &syn::ItemImpl) -> Option<Vec<String>> {}
    pub(crate) fn module_hints(
        rel_path: &Path,
        name: &str,
        item_path: Option<&[String]>,
        project: &Project,
    ) -> Vec<Vec<String>> {
    }
    pub(crate) fn impl_file_module_path(rel_path: &Path) -> Vec<String> {}
    pub(crate) fn defining_module_path(
        qualifier: &[String],
        anchor: &[String],
    ) -> Option<Vec<String>> {
    }
    pub(crate) fn import_module_hint(
        prefix: &[String],
        imported: &syn::Ident,
        type_name: &str,
        anchor: &[String],
    ) -> Option<Vec<String>> {
    }
    pub(crate) fn candidate_defines_module_path(candidate: &Path, module_path: &[String]) -> bool {}
    pub(crate) fn narrowed_candidates(
        files: &std::collections::BTreeSet<std::path::PathBuf>,
        hints: &[Vec<String>],
    ) -> Vec<std::path::PathBuf> {
    }
}

// no test_usage necessary
