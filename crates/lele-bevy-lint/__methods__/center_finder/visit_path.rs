use crate::scan::center_finder::CenterFinder;

const CENTERING_FNS: [&str; 2] = ["centered_root", "center_roots"];

pub fn visit_path(this: &mut CenterFinder<'_>, node: &syn::Path) {
    if node
        .segments
        .last()
        .is_some_and(|segment| CENTERING_FNS.iter().any(|name| segment.ident == name))
    {
        ***this = true;
    }
    syn::visit::visit_path(this, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::CenterFinder;

    #[test]
    fn test_usage() {
        let mut found = false;
        let mut visitor = CenterFinder(&mut found);
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
