use crate::rules;
use crate::Category;
use crate::ExampleFile;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use crate::Visibility;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;
use std::path::PathBuf;

pub struct ContainerPlacement;

impl ContainerPlacement {
    pub const NAME: &'static str = "container_placement";
    pub const CODE: &'static str = "LELE020";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "Behavior-free types (plain structs, enums, newtypes, aliases, ECS markers) go in the domain's `__basic__/<role>.rs`.",
        why: "Small data types don't need a file and a test each; grouping them keeps atomic files for code that does something.",
        fix: "Move the behavior-free type into its domain's `__basic__/<role>.rs` (`structs.rs`, `enums.rs`, ...) and import it as `basic::<Type>`; a type with an `impl` keeps its own file.",
        bad: rule_examples!("container_placement/bad"),
        good: rule_examples!("container_placement/good"),
    };
}

#[rustfmt::skip]
impl Rule for ContainerPlacement {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ContainerPlacement)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ContainerPlacement {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ContainerPlacement));
    }
    pub(crate) fn role_of_stem(stem: &str) -> Option<rules::container_placement::Role> {
        rules::container_placement::Role::ALL
            .into_iter()
            .find(|role| role.file().strip_suffix(".rs") == Some(stem))
    }
    pub(crate) fn exposed_name(item: &syn::Item) -> Option<String> {
        match item {
            syn::Item::Struct(s) if Visibility(&s.vis).is_exposed() => Some(s.ident.to_string()),
            syn::Item::Enum(e) if Visibility(&e.vis).is_exposed() => Some(e.ident.to_string()),
            syn::Item::Type(t) if Visibility(&t.vis).is_exposed() => Some(t.ident.to_string()),
            syn::Item::Const(c) if Visibility(&c.vis).is_exposed() => Some(c.ident.to_string()),
            syn::Item::Static(s) if Visibility(&s.vis).is_exposed() => Some(s.ident.to_string()),
            _ => None,
        }
    }
    pub(crate) fn classify(item: &syn::Item) -> Option<rules::container_placement::Role> {
        match item {
            syn::Item::Struct(s) => {
                let derives = Self::derive_names(&s.attrs);
                if Self::is_excluded(&derives) {
                    return None;
                }
                if let Some(role) = Self::ecs_role(&derives) {
                    return Some(role);
                }
                if matches!(s.fields, syn::Fields::Unnamed(ref f) if f.unnamed.len() == 1) {
                    Some(rules::container_placement::Role::Newtype)
                } else {
                    Some(rules::container_placement::Role::Struct)
                }
            }
            syn::Item::Enum(e) => {
                let derives = Self::derive_names(&e.attrs);
                if Self::is_excluded(&derives) {
                    return None;
                }
                Self::ecs_role(&derives).or(Some(rules::container_placement::Role::Enum))
            }
            syn::Item::Type(_) => Some(rules::container_placement::Role::TypeAlias),
            syn::Item::Const(_) | syn::Item::Static(_) => Some(rules::container_placement::Role::Const),
            _ => None,
        }
    }
    pub(crate) fn is_excluded(derives: &[String]) -> bool {
        derives.iter().any(|derive| {
            matches!(
                derive.as_str(),
                "Error" | "Parser" | "Args" | "Subcommand" | "ValueEnum" | "NetworkBehaviour"
            )
        })
    }
    pub(crate) fn has_behavior(file: &syn::File, type_name: &str) -> bool {
        file.items.iter().any(|item| {
            let syn::Item::Impl(impl_block) = item else {
                return false;
            };
            if impl_block
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_atomic_delegates())
            {
                return true;
            }
            ImplBlock(impl_block).self_type_last().as_deref() == Some(type_name)
        })
    }
}

#[atomic_delegates]
impl ContainerPlacement {
    pub(crate) fn check_container_file(
        rel_path: &Path,
        file: &syn::File,
        project: &Project,
        file_path: &Path,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_free_file(
        rel_path: &Path,
        file: &syn::File,
        folder: &str,
        file_path: &Path,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn ecs_role(derives: &[String]) -> Option<rules::container_placement::Role> {}
    pub(crate) fn derive_names(attrs: &[syn::Attribute]) -> Vec<String> {}
    pub(crate) fn container_dir(rel_path: &Path, folder: &str) -> PathBuf {}
}

// no test_usage necessary
