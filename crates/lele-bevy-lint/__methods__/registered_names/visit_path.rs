use crate::scan;

pub fn visit_path(
    registered_names: &mut scan::registered_names::RegisteredNames<'_, '_>,
    node: &syn::Path,
) {
    if let Some(last) = node.segments.last() {
        registered_names.push(last.ident.to_string());
    }
    syn::visit::visit_path(registered_names, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use crate::scan::registered_names::RegisteredNames;
    use crate::scan::use_collector::UseCollector;

    #[test]
    fn test_usage() {
        let mut names = Vec::new();
        let mut collector = UseCollector(&mut names);
        let mut visitor = RegisteredNames(&mut collector);
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
