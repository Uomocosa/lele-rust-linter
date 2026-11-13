use crate::rules;
use syn::visit::Visit;

pub fn inspect_signature(
    visitor: &mut rules::no_stringified_errors::StringifiedErrorVisitor<'_>,
    sig: &syn::Signature,
    block: &syn::Block,
) {
    for input in &sig.inputs {
        let syn::FnArg::Typed(typed) = input else {
            continue;
        };
        let syn::Pat::Ident(pat) = &*typed.pat else {
            continue;
        };
        if visitor.is_from_impl_for_error_enum
            || rules::no_stringified_errors::NoStringifiedErrors::names_an_error_type(&typed.ty)
        {
            visitor.inspect(&pat.ident, |scan| scan.visit_block(block));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::collections::HashSet;

    #[test]
    fn test_usage() {
        let error_enums = HashSet::from(["Failure".to_string()]);
        let mut hits = Vec::new();
        let mut visitor = rules::no_stringified_errors::StringifiedErrorVisitor {
            error_enums: &error_enums,
            impl_self: None,
            is_from_impl_for_error_enum: false,
            hits: &mut hits,
        };
        let sig: syn::Signature = syn::parse_quote! { fn wrap(err: &std::io::Error) -> Failure };
        let block: syn::Block = syn::parse_quote! { { Failure::Io(err.to_string()) } };
        rules::no_stringified_errors::StringifiedErrorVisitor::inspect_signature(
            &mut visitor,
            &sig,
            &block,
        );
        assert_eq!(visitor.hits.len(), 1);
    }
}
