use crate::rules;
use atomic_delegate_macros::atomic_delegates;
use std::collections::HashSet;
use syn::visit::Visit;

pub(crate) struct StringifiedErrorVisitor<'a> {
    pub(crate) error_enums: &'a HashSet<String>,
    pub(crate) impl_self: Option<String>,
    pub(crate) is_from_impl_for_error_enum: bool,
    pub(crate) hits: &'a mut Vec<rules::no_stringified_errors::StringifiedErrorHit>,
}

#[atomic_delegates]
impl<'ast> Visit<'ast> for StringifiedErrorVisitor<'_> {
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {}
    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {}
    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {}
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {}
}

impl StringifiedErrorVisitor<'_> {
    pub(crate) fn inspect(
        &mut self,
        error_name: &syn::Ident,
        body: impl FnOnce(&mut rules::no_stringified_errors::CaughtErrorScan<'_>),
    ) {
        let mut scan = rules::no_stringified_errors::CaughtErrorScan {
            error_name: error_name.to_string(),
            error_enums: self.error_enums,
            impl_self: self.impl_self.as_deref(),
            built_variant: None,
            is_stringified: false,
            is_kept: false,
        };
        body(&mut scan);
        if let (Some(variant), true, false) =
            (scan.built_variant, scan.is_stringified, scan.is_kept)
        {
            self.hits
                .push(rules::no_stringified_errors::StringifiedErrorHit {
                    line: error_name.span().start().line,
                    error_name: scan.error_name,
                    variant,
                });
        }
    }
}

#[atomic_delegates]
impl StringifiedErrorVisitor<'_> {
    pub(crate) fn inspect_signature(&mut self, sig: &syn::Signature, block: &syn::Block) {}
}

// no test_usage necessary
