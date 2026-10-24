use syn::visit::Visit;

use crate::scan;
use crate::scan::basic::constants::HARNESS_RUN;

pub(crate) struct HarnessScanner {
    pub(crate) is_run_imported: bool,
    pub(crate) is_called: bool,
    pub(crate) is_checked: bool,
}

const CHECKING_METHODS: [&str; 2] = ["expect", "unwrap"];

#[rustfmt::skip]
impl<'ast> Visit<'ast> for HarnessScanner {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if scan::HarnessCall::is_harness_path(&node.func, self.is_run_imported) {
            self.is_called = true;
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if node.method == HARNESS_RUN {
            self.is_called = true;
        }
        let is_checking = CHECKING_METHODS.iter().any(|method| node.method == method);
        if is_checking && scan::HarnessCall::is_harness_run(&node.receiver, self.is_run_imported) {
            self.is_checked = true;
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_try(&mut self, node: &'ast syn::ExprTry) {
        if scan::HarnessCall::is_harness_run(&node.expr, self.is_run_imported) {
            self.is_checked = true;
        }
        syn::visit::visit_expr_try(self, node);
    }

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
        let body: syn::Block = syn::parse_quote! {{ scene.run(&c, "x").expect("preview"); }};
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
