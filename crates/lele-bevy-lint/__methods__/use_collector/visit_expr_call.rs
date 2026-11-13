use crate::scan::use_collector::UseCollector;

pub fn visit_expr_call(this: &mut UseCollector<'_>, node: &syn::ExprCall) {
    if let syn::Expr::Path(path) = &*node.func
        && let Some(last) = path.path.segments.last()
    {
        this.push(last.ident.to_string());
    }
    syn::visit::visit_expr_call(this, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::UseCollector;

    #[test]
    fn test_usage() {
        let mut out = Vec::new();
        let mut visitor = UseCollector(&mut out);
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
