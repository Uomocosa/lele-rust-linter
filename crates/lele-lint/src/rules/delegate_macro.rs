use crate::Category;
use crate::ExampleFile;
use crate::FnBody;
use crate::ImplBlock;
use crate::ImplItemGroup;
use crate::ItemMarker;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use lele_snake_case::ToLeleSnakeCase;
use rule_examples_macro::rule_examples;
use std::collections::HashSet;
use std::path::Path;

const RESERVED_DELEGATE_METHODS: &[&str] = &["new"];

pub struct DelegateMacro;

impl DelegateMacro {
    pub const NAME: &'static str = "delegate_macro";
    pub const CODE: &'static str = "LELE032";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "Method shells (empty bodies) are declared in one `#[atomic_delegates]` impl block per `impl_item_order` group — constructor shells, `pub` shells, `pub(crate)` shells — and no block mixes groups; `new` is never delegated.",
        why: "The macro wires each shell to `__methods__/<type>/<method>.rs`; one block per `impl_item_order` group lets hand-written impls sit between the type's shell blocks, in the order `impl_item_order` wants: constructors, then trait impls, then `pub` fns.",
        fix: "Split the empty-bodied shells by their `impl_item_order` group into one `#[atomic_delegates] impl <Type>` block per group (constructor shells, `pub` shells, `pub(crate)` shells), keep hand-written methods in a plain block, and never delegate `new`.",
        bad: rule_examples!("delegate_macro/bad"),
        good: rule_examples!("delegate_macro/good"),
    };
}

#[rustfmt::skip]
impl Rule for DelegateMacro {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(DelegateMacro)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl DelegateMacro {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(DelegateMacro));
    }
    pub(crate) fn mirror_names(project: &Project, primary: &str, idents: &[&syn::Ident]) -> Vec<String> {
        let root = project.methods_root_name();
        idents
            .iter()
            .map(|ident| format!("{root}/{}/{ident}.rs", primary.to_snake_case()))
            .collect()
    }
    pub(crate) fn annotated_inherent_blocks(file: &syn::File, primary: &str, group: ImplItemGroup) -> usize {
        file.items
            .iter()
            .filter(|item| match item {
                syn::Item::Impl(block) => {
                    block.trait_.is_none()
                        && !block
                            .attrs
                            .iter()
                            .any(|attr| ItemMarker(attr).is_cfg_test())
                        && block
                            .attrs
                            .iter()
                            .any(|attr| ItemMarker(attr).is_atomic_delegates())
                        && ImplBlock(block).self_type_last().as_deref() == Some(primary)
                        && Self::shell_order_groups(block).as_slice() == [group]
                }
                _ => false,
            })
            .count()
    }
    pub(crate) fn has_fn(impl_block: &syn::ItemImpl) -> bool {
        impl_block
            .items
            .iter()
            .any(|item| matches!(item, syn::ImplItem::Fn(_)))
    }
    pub(crate) fn migratable_idents(impl_block: &syn::ItemImpl) -> Vec<&syn::Ident> {
        impl_block
            .items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(method)
                    if !method
                        .attrs
                        .iter()
                        .any(|attr| ItemMarker(attr).is_atomic_delegate())
                        && (method.block.stmts.is_empty()
                            || FnBody(&method.block).is_delegate_call()
                            || FnBody(&method.block).is_methods_dispatch()) =>
                {
                    Some(&method.sig.ident)
                }
                _ => None,
            })
            .collect()
    }
    pub(crate) fn real_idents(impl_block: &syn::ItemImpl) -> Vec<&syn::Ident> {
        impl_block
            .items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(method)
                    if !method
                        .attrs
                        .iter()
                        .any(|attr| ItemMarker(attr).is_atomic_delegate())
                        && !method.block.stmts.is_empty()
                        && !FnBody(&method.block).is_delegate_call()
                        && !FnBody(&method.block).is_methods_dispatch() =>
                {
                    Some(&method.sig.ident)
                }
                _ => None,
            })
            .collect()
    }
    pub(crate) fn fn_methods(impl_block: &syn::ItemImpl) -> Vec<&syn::ImplItemFn> {
        impl_block
            .items
            .iter()
            .filter_map(|item| match item {
                syn::ImplItem::Fn(method) => Some(method),
                _ => None,
            })
            .collect()
    }
    pub(crate) fn is_reserved(ident: &syn::Ident) -> bool {
        RESERVED_DELEGATE_METHODS.contains(&ident.to_string().as_str())
    }
}

#[atomic_delegates]
impl DelegateMacro {
    pub(crate) fn check_delegated_methods(
        project: &Project,
        rel_path: &Path,
        primary: &str,
        impl_block: &syn::ItemImpl,
        seen: &mut HashSet<String>,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_fn_shells(
        project: &Project,
        rel_path: &Path,
        primary: &str,
        impl_block: &syn::ItemImpl,
        seen: &mut HashSet<String>,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn all_methods_are_real_reserved(impl_block: &syn::ItemImpl) -> bool {}
    pub(crate) fn shell_order_groups(impl_block: &syn::ItemImpl) -> Vec<ImplItemGroup> {}
}

// no test_usage necessary
