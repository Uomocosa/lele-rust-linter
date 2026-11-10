use crate::rules;

pub fn record_variant(
    scan: &mut rules::no_stringified_errors::CaughtErrorScan<'_>,
    path: &syn::Path,
) {
    let segments: Vec<String> = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    let [.., owner, variant] = segments.as_slice() else {
        return;
    };
    let owner = if owner == "Self" {
        scan.impl_self.unwrap_or_default()
    } else {
        owner.as_str()
    };
    if scan.error_enums.contains(owner) && scan.built_variant.is_none() {
        scan.built_variant = Some(format!("{owner}::{variant}"));
    }
}

// no test_usage necessary
