use crate::rules;
use crate::rules::no_crate_paths::crate_path_visitor::CratePathVisitor;
use syn::spanned::Spanned;

pub fn visit_path(this: &mut CratePathVisitor<'_>, path: &syn::Path) {
    if path
        .segments
        .first()
        .is_some_and(|seg| seg.ident == "crate")
    {
        let path_str = path
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        this.push(rules::no_crate_paths::CratePathHit {
            line: path.span().start().line,
            path: path_str,
        });
    }
    syn::visit::visit_path(this, path);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::CratePathVisitor;

    #[test]
    fn test_usage() {
        let mut hits = Vec::new();
        let mut visitor = CratePathVisitor(&mut hits);
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
