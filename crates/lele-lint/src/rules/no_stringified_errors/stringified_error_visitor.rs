use crate::rules;
use std::collections::HashSet;
use syn::visit::Visit;

pub(crate) struct StringifiedErrorVisitor<'a> {
    pub(crate) error_enums: &'a HashSet<String>,
    pub(crate) impl_self: Option<String>,
    pub(crate) is_from_impl_for_error_enum: bool,
    pub(crate) hits: &'a mut Vec<rules::no_stringified_errors::StringifiedErrorHit>,
}

impl<'ast> Visit<'ast> for StringifiedErrorVisitor<'_> {
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        if call.method == "map_err" {
            if let Some(syn::Expr::Closure(closure)) = call.args.first() {
                if let [syn::Pat::Ident(pat)] = closure.inputs.iter().collect::<Vec<_>>().as_slice()
                {
                    self.inspect(&pat.ident, |scan| scan.visit_expr(&closure.body));
                }
            }
        }
        syn::visit::visit_expr_method_call(self, call);
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        let previous_self = self.impl_self.take();
        let previous_from = self.is_from_impl_for_error_enum;
        self.impl_self =
            rules::no_stringified_errors::NoStringifiedErrors::last_ident(&item.self_ty);
        self.is_from_impl_for_error_enum = item.trait_.as_ref().is_some_and(|(_, path, _)| {
            path.segments
                .last()
                .is_some_and(|segment| segment.ident == "From")
        }) && self
            .impl_self
            .as_ref()
            .is_some_and(|name| self.error_enums.contains(name));
        syn::visit::visit_item_impl(self, item);
        self.impl_self = previous_self;
        self.is_from_impl_for_error_enum = previous_from;
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        self.inspect_signature(&item.sig, &item.block);
        syn::visit::visit_impl_item_fn(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let previous_from = self.is_from_impl_for_error_enum;
        self.is_from_impl_for_error_enum = false;
        self.inspect_signature(&item.sig, &item.block);
        syn::visit::visit_item_fn(self, item);
        self.is_from_impl_for_error_enum = previous_from;
    }
}

#[rustfmt::skip]
impl StringifiedErrorVisitor<'_> {
    pub(crate) fn inspect(&mut self, error_name: &syn::Ident, body: impl FnOnce(&mut rules::no_stringified_errors::CaughtErrorScan<'_>)) {
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
            self.hits.push(rules::no_stringified_errors::StringifiedErrorHit {
                line: error_name.span().start().line,
                error_name: scan.error_name,
                variant,
            });
        }
    }

    pub(crate) fn inspect_signature(&mut self, sig: &syn::Signature, block: &syn::Block) {
        for input in &sig.inputs {
            let syn::FnArg::Typed(typed) = input else {
                continue;
            };
            let syn::Pat::Ident(pat) = &*typed.pat else {
                continue;
            };
            if self.is_from_impl_for_error_enum || rules::no_stringified_errors::NoStringifiedErrors::names_an_error_type(&typed.ty) {
                self.inspect(&pat.ident, |scan| scan.visit_block(block));
            }
        }
    }
}

// no test_usage necessary
