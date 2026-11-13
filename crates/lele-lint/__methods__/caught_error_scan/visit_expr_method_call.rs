use crate::rules::no_stringified_errors::caught_error_scan::CaughtErrorScan;

pub fn visit_expr_method_call(this: &mut CaughtErrorScan<'_>, call: &syn::ExprMethodCall) {
    if call.method == "to_string" && this.is_error_name(&call.receiver) {
        this.is_stringified = true;
        return;
    }
    syn::visit::visit_expr_method_call(this, call);
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::CaughtErrorScan;

    #[test]
    fn test_usage() {
        let error_enums = std::collections::HashSet::new();
        let mut visitor = CaughtErrorScan {
            error_name: String::from("Error"),
            error_enums: &error_enums,
            impl_self: None,
            built_variant: None,
            is_stringified: false,
            is_kept: false,
        };
        let file: syn::File = syn::parse_str("mod m { pub fn f() {} }\npub(crate) trait T { fn t(&self); }\nuse a::b::c;\nuse a::d as e;\n#[allow(dead_code)]\npub(crate) fn f(x: &u8) -> u8 { let y = g(1, 2); let s = S { a: 1 }; let z = x.to_string(); let w = y.m(); vec![1]; crate::a::b(); let q = w?; self::m(); let u = a::b::c; q }\nimpl X { fn g(&self) {} }\nimpl T for X { fn t(&self) { self.g(); Self::g(self); } }\n").unwrap();
        visitor.visit_file(&file);
    }
}
