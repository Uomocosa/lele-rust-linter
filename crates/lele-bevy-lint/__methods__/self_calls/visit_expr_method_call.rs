use crate::scan::self_calls::SelfCalls;

pub fn visit_expr_method_call(this: &mut SelfCalls<'_>, node: &syn::ExprMethodCall) {
    if let syn::Expr::Path(receiver) = &*node.receiver
        && receiver.path.is_ident("self")
    {
        this.push(node.method.to_string());
    }
    syn::visit::visit_expr_method_call(this, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::SelfCalls;

    #[test]
    fn test_usage() {
        let mut out = Vec::new();
        let mut visitor = SelfCalls(&mut out);
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
