use atomic_delegate_macros::atomic_delegates;

use crate::rules;
use std::collections::HashSet;
use syn::visit::Visit;

pub(crate) struct CaughtErrorScan<'a> {
    pub(crate) error_name: String,
    pub(crate) error_enums: &'a HashSet<String>,
    pub(crate) impl_self: Option<&'a str>,
    pub(crate) built_variant: Option<String>,
    pub(crate) is_stringified: bool,
    pub(crate) is_kept: bool,
}

#[rustfmt::skip]
impl<'ast> Visit<'ast> for CaughtErrorScan<'_> {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*call.func {
            self.record_variant(&path.path);
        }
        syn::visit::visit_expr_call(self, call);
    }

    fn visit_expr_struct(&mut self, item: &'ast syn::ExprStruct) {
        self.record_variant(&item.path);
        syn::visit::visit_expr_struct(self, item);
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        if call.method == "to_string" && self.is_error_name(&call.receiver) {
            self.is_stringified = true;
            return;
        }
        syn::visit::visit_expr_method_call(self, call);
    }

    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        if path.path.is_ident(&self.error_name) {
            self.is_kept = true;
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if rules::no_stringified_errors::NoStringifiedErrors::mentions_name(mac.tokens.clone(), &self.error_name) {
            self.is_stringified = true;
        }
    }
}

#[atomic_delegates]
impl CaughtErrorScan<'_> {
    pub(crate) fn record_variant(&mut self, path: &syn::Path) {}
}

#[rustfmt::skip]
impl CaughtErrorScan<'_> {
    pub(crate) fn is_error_name(&self, expr: &syn::Expr) -> bool {
        matches!(expr, syn::Expr::Path(path) if path.path.is_ident(&self.error_name))
    }
}

// no test_usage necessary
