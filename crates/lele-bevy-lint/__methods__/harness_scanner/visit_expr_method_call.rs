use crate::scan;
use crate::scan::harness_scanner::HarnessScanner;

const CHECKING_METHODS: [&str; 2] = ["expect", "unwrap"];

pub fn visit_expr_method_call(this: &mut HarnessScanner, node: &syn::ExprMethodCall) {
    if node.method == scan::HARNESS_RUN {
        this.is_called = true;
    }
    let is_checking = CHECKING_METHODS.iter().any(|method| node.method == method);
    if is_checking && scan::HarnessCall::is_harness_run(&node.receiver, this.is_run_imported) {
        this.is_checked = true;
    }
    syn::visit::visit_expr_method_call(this, node);
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
