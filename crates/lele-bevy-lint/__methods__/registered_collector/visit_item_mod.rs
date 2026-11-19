use crate::scan::registered_collector::RegisteredCollector;
use lele_lint::ItemMarker;

pub fn visit_item_mod(this: &mut RegisteredCollector<'_>, node: &syn::ItemMod) {
    if node.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
        return;
    }
    syn::visit::visit_item_mod(this, node);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::RegisteredCollector;

    #[test]
    fn test_usage() {
        let mut out = Vec::new();
        let mut visitor = RegisteredCollector(&mut out);
        let item: syn::Item = syn::parse_str(
            "mod ui { pub fn build(app: &mut App) { app.add_systems(Update, tick); } }",
        )
        .unwrap();
        let syn::Item::Mod(module) = item else {
            panic!("expected a module");
        };
        super::visit_item_mod(&mut visitor, &module);
        assert!(visitor.contains(&"tick".to_string()));
    }

    #[test]
    fn visit_file_walks_every_item() {
        let mut out = Vec::new();
        let mut visitor = RegisteredCollector(&mut out);
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
