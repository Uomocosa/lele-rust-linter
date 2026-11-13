use crate::scan;
use crate::scan::call_collector::CallCollector;
use syn::visit::Visit;

pub fn visit_expr_method_call(this: &mut CallCollector<'_>, node: &syn::ExprMethodCall) {
    if let Some(idents) = scan::AppMethodCall(node).system_args() {
        this.calls.extend(idents.iter().cloned());
        this.systems.extend(idents);
        this.visit_expr(&node.receiver);
    } else if let Some(idents) = scan::AppMethodCall(node).observer_args() {
        this.calls.extend(idents.iter().cloned());
        this.systems.extend(idents);
        this.visit_expr(&node.receiver);
    } else {
        syn::visit::visit_expr_method_call(this, node);
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::CallCollector;

    #[test]
    fn test_usage() {
        let mut calls = Vec::new();
        let mut systems = Vec::new();
        let mut visitor = CallCollector {
            calls: &mut calls,
            systems: &mut systems,
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
