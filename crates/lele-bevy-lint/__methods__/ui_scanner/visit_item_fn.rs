use crate::scan::ui_scanner::UiScanner;

pub fn visit_item_fn(this: &mut UiScanner<'_, '_>, node: &syn::ItemFn) {
    let outer = std::mem::take(this.owner);
    *this.owner = node.sig.ident.to_string();
    syn::visit::visit_item_fn(this, node);
    *this.owner = outer;
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::UiScanner;

    #[test]
    fn test_usage() {
        let mut found = Vec::new();
        let mut owner = String::new();
        let module_fns = std::collections::HashMap::new();
        let extra = std::collections::BTreeSet::new();
        let mut visitor = UiScanner {
            found: &mut found,
            owner: &mut owner,
            module_fns: &module_fns,
            extra: &extra,
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
