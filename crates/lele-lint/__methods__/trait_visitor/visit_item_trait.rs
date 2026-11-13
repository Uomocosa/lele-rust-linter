use crate::rules;
use crate::rules::trait_placement::trait_visitor::TraitVisitor;
use crate::ItemMarker;

pub fn visit_item_trait(this: &mut TraitVisitor<'_>, item: &syn::ItemTrait) {
    let in_test = item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test());
    if !in_test {
        this.push(rules::trait_placement::TraitHit {
            line: item.ident.span().start().line,
            name: item.ident.to_string(),
        });
    }
    syn::visit::visit_item_trait(this, item);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::TraitVisitor;

    #[test]
    fn test_usage() {
        let mut hits = Vec::new();
        let mut visitor = TraitVisitor(&mut hits);
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
