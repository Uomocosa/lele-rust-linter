use crate::rules;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;

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

impl<'ast> Visit<'ast> for CaughtErrorScan<'_> {
    #[atomic_delegate(CaughtErrorScan, Visit)]
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {}
    #[atomic_delegate(CaughtErrorScan, Visit)]
    fn visit_expr_struct(&mut self, item: &'ast syn::ExprStruct) {}
    #[atomic_delegate(CaughtErrorScan, Visit)]
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {}
    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        if path.path.is_ident(&self.error_name) {
            self.is_kept = true;
        }
    }
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if rules::no_stringified_errors::NoStringifiedErrors::mentions_name(
            mac.tokens.clone(),
            &self.error_name,
        ) {
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
