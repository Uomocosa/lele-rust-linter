use crate::scan;
use crate::scan::styling_detector::StylingDetector;

pub fn visit_type_reference(this: &mut StylingDetector<'_>, node: &syn::TypeReference) {
    if node.mutability.is_some() && scan::ScannedFile::is_styling(&node.elem) {
        this.found = true;
    }
    syn::visit::visit_type_reference(this, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::StylingDetector;

    #[test]
    fn test_usage() {
        let aliases = std::collections::HashSet::new();
        let mut visitor = StylingDetector {
            aliases: &aliases,
            found: false,
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
