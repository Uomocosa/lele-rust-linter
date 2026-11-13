use crate::scan;
use crate::scan::scene_finder::SceneFinder;

pub fn visit_expr_struct(this: &mut SceneFinder<'_>, node: &syn::ExprStruct) {
    let literal = scan::StructLiteral(node);
    if literal.is_type("Scene") && literal.has_field("build") {
        this.push(node.clone());
    }
    syn::visit::visit_expr_struct(this, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::SceneFinder;

    #[test]
    fn test_usage() {
        let mut out = Vec::new();
        let mut visitor = SceneFinder(&mut out);
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
