use syn::visit::Visit;

use crate::scan;
use crate::scan::basic::constants::{HARNESS_CRATE, HARNESS_RUN};

const CHECKING_METHODS: [&str; 2] = ["expect", "unwrap"];

#[must_use]
pub fn detect(body: &syn::Block, is_run_imported: bool) -> scan::HarnessCall {
    let mut scanner = HarnessScanner {
        is_run_imported,
        is_called: false,
        is_checked: false,
    };
    scanner.visit_block(body);
    match (scanner.is_called, scanner.is_checked) {
        (false, _) => scan::HarnessCall::Missing,
        (true, false) => scan::HarnessCall::Unchecked,
        (true, true) => scan::HarnessCall::Checked,
    }
}

// needed helper: records the harness call and whether its Result is checked
struct HarnessScanner {
    is_run_imported: bool,
    is_called: bool,
    is_checked: bool,
}

impl<'ast> Visit<'ast> for HarnessScanner {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if is_harness_path(&node.func, self.is_run_imported) {
            self.is_called = true;
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let is_checking = CHECKING_METHODS.iter().any(|method| node.method == method);
        if is_checking && is_harness_run(&node.receiver, self.is_run_imported) {
            self.is_checked = true;
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_try(&mut self, node: &'ast syn::ExprTry) {
        if is_harness_run(&node.expr, self.is_run_imported) {
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

// needed helper: is this expression (through parens) a harness `run(...)` call?
fn is_harness_run(expr: &syn::Expr, is_run_imported: bool) -> bool {
    match expr {
        syn::Expr::Paren(inner) => is_harness_run(&inner.expr, is_run_imported),
        syn::Expr::Group(inner) => is_harness_run(&inner.expr, is_run_imported),
        syn::Expr::Call(call) => is_harness_path(&call.func, is_run_imported),
        _ => false,
    }
}

// needed helper: `lele_bevy_preview::run`, or a bare `run` imported from it
fn is_harness_path(func: &syn::Expr, is_run_imported: bool) -> bool {
    let syn::Expr::Path(path) = func else {
        return false;
    };
    let segments = &path.path.segments;
    let Some(last) = segments.last() else {
        return false;
    };
    if last.ident != HARNESS_RUN {
        return false;
    }
    if segments.len() == 1 {
        return is_run_imported;
    }
    segments
        .iter()
        .any(|segment| segment.ident == HARNESS_CRATE)
}

#[cfg(test)]
mod tests {
    use super::detect;
    use crate::scan::HarnessCall;

    fn call(body: &str, is_run_imported: bool) -> HarnessCall {
        detect(&syn::parse_str(body).unwrap(), is_run_imported)
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            call(
                "{ lele_bevy_preview::run(&s, &c, \"x\").expect(\"preview\"); }",
                false
            ),
            HarnessCall::Checked
        );
        assert_eq!(call("{ run(&s, &c, \"x\")?; }", true), HarnessCall::Checked);
        assert_eq!(
            call("{ let _ = run(&s, &c, \"x\"); }", true),
            HarnessCall::Unchecked
        );
        assert_eq!(
            call("{ run(&s, &c, \"x\").expect(\"p\"); }", false),
            HarnessCall::Missing
        );
        assert_eq!(call("{ server::run(); }", true), HarnessCall::Missing);
    }
}
