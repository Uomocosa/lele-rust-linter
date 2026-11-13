use crate::scan;
use crate::scan::harness_scanner::HarnessScanner;

pub fn visit_expr_try(this: &mut HarnessScanner, node: &syn::ExprTry) {
    if scan::HarnessCall::is_harness_run(&node.expr, this.is_run_imported) {
        this.is_checked = true;
    }
    syn::visit::visit_expr_try(this, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::HarnessScanner;

    #[test]
    fn test_usage() {
        let mut visitor = HarnessScanner {
            is_run_imported: false,
            is_called: false,
            is_checked: false,
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
