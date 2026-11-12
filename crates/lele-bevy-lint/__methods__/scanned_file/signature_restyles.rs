use std::collections::HashSet;

use syn::visit::Visit;

use crate::scan;

pub fn signature_restyles(sig: &syn::Signature, aliases: &HashSet<String>) -> bool {
    sig.inputs.iter().any(|input| {
        let syn::FnArg::Typed(pat) = input else {
            return false;
        };
        let mut detector = scan::StylingDetector {
            aliases,
            found: false,
        };
        detector.visit_type(&pat.ty);
        detector.found
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::signature_restyles;

    #[test]
    fn test_usage() {
        let aliases = HashSet::new();
        let restyle: syn::Signature = syn::parse_quote! { fn tint(node: &mut Node) };
        let plain: syn::Signature = syn::parse_quote! { fn count(total: &u32) };
        assert!(signature_restyles(&restyle, &aliases));
        assert!(!signature_restyles(&plain, &aliases));
    }
}
