use crate::rules;
use crate::Identifier;
use std::collections::HashSet;
use syn::spanned::Spanned;

pub fn collect_stutter(
    path: &syn::Path,
    root_stems: &HashSet<String>,
    hits: &mut Vec<rules::no_stuttered_path::StutterHit>,
) {
    if path.leading_colon.is_some() {
        return;
    }
    let segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
    if segments
        .first()
        .is_some_and(|s| matches!(s.as_str(), "crate" | "std" | "core" | "alloc"))
    {
        return;
    }
    for pair in segments.windows(2) {
        if let [module, ty] = pair {
            if root_stems.contains(module) && Identifier(ty).is_stuttered_in(module) {
                hits.push(rules::no_stuttered_path::StutterHit {
                    line: path.span().start().line,
                    path: segments.join("::"),
                    ty: ty.clone(),
                });
                return;
            }
        }
    }
}

// no test_usage necessary
