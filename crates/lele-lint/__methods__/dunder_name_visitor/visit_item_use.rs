use crate::rules;
use crate::rules::dunder_module_name::dunder_name_visitor::DunderNameVisitor;
use crate::Visibility;

pub fn visit_item_use(this: &mut DunderNameVisitor<'_>, item: &syn::ItemUse) {
    if Visibility(&item.vis).is_exposed() {
        rules::dunder_module_name::DunderModuleName::collect_reexports(this, item);
    }
    syn::visit::visit_item_use(this, item);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::DunderNameVisitor;

    #[test]
    fn test_usage() {
        let mut hits = Vec::new();
        let mut reexports = Vec::new();
        let mut visitor = DunderNameVisitor {
            dunders: &[],
            hits: &mut hits,
            reexports: &mut reexports,
            domain_prefix: Vec::new(),
            methods_mount_name: String::new(),
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
