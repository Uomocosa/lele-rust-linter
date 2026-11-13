use crate::scan;
use atomic_delegate_macros::atomic_delegate;
use syn::visit::Visit;

pub(crate) struct HarnessScanner {
    pub(crate) is_run_imported: bool,
    pub(crate) is_called: bool,
    pub(crate) is_checked: bool,
}

impl<'ast> Visit<'ast> for HarnessScanner {
    #[atomic_delegate(HarnessScanner, Visit)]
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {}
    #[atomic_delegate(HarnessScanner, Visit)]
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {}
    #[atomic_delegate(HarnessScanner, Visit)]
    fn visit_expr_try(&mut self, node: &'ast syn::ExprTry) {}
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::MacroCall(node).exprs() {
            self.visit_expr(&expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::HarnessScanner;

    #[test]
    fn test_usage() {
        let body: syn::Block = syn::parse_quote! {{ scene.run(&c).expect("preview"); }};
        let mut scanner = HarnessScanner {
            is_run_imported: false,
            is_called: false,
            is_checked: false,
        };
        scanner.visit_block(&body);
        assert!(scanner.is_called);
        assert!(scanner.is_checked);
    }
}
