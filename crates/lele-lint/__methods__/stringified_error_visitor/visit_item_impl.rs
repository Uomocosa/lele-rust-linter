use crate::rules;
use crate::rules::no_stringified_errors::stringified_error_visitor::StringifiedErrorVisitor;

pub fn visit_item_impl(this: &mut StringifiedErrorVisitor<'_>, item: &syn::ItemImpl) {
    let previous_self = this.impl_self.take();
    let previous_from = this.is_from_impl_for_error_enum;
    this.impl_self = rules::no_stringified_errors::NoStringifiedErrors::last_ident(&item.self_ty);
    this.is_from_impl_for_error_enum = item.trait_.as_ref().is_some_and(|(_, path, _)| {
        path.segments
            .last()
            .is_some_and(|segment| segment.ident == "From")
    }) && this
        .impl_self
        .as_ref()
        .is_some_and(|name| this.error_enums.contains(name));
    syn::visit::visit_item_impl(this, item);
    this.impl_self = previous_self;
    this.is_from_impl_for_error_enum = previous_from;
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::StringifiedErrorVisitor;

    #[test]
    fn test_usage() {
        let error_enums = std::collections::HashSet::new();
        let mut hits = Vec::new();
        let mut visitor = StringifiedErrorVisitor {
            error_enums: &error_enums,
            impl_self: None,
            is_from_impl_for_error_enum: false,
            hits: &mut hits,
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
