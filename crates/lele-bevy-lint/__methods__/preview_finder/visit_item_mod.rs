use crate::scan;
use crate::scan::preview_finder::PreviewFinder;

pub fn visit_item_mod<'ast>(this: &mut PreviewFinder<'_, 'ast>, node: &'ast syn::ItemMod) {
    let Some((_, items)) = &node.content else {
        return;
    };
    let outer = std::mem::replace(
        &mut this.module_fns,
        scan::ScannedFile::module_fns_of(items),
    );
    syn::visit::visit_item_mod(this, node);
    this.module_fns = outer;
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::PreviewFinder;

    #[test]
    fn test_usage() {
        let mut previews = Vec::new();
        let module_fns = std::collections::HashMap::new();
        let mut visitor = PreviewFinder {
            previews: &mut previews,
            is_run_imported: false,
            module_fns,
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
