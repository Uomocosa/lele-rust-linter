use crate::rules;
use crate::Identifier;
use std::collections::HashSet;

pub fn missing_reexports(
    stem: &str,
    file: &syn::File,
    reexported: &HashSet<(String, String)>,
) -> Vec<rules::root_reexport::MissingReexport> {
    let mut missing = Vec::new();
    for item in &file.items {
        let (name, span) = match item {
            syn::Item::Struct(s) if matches!(s.vis, syn::Visibility::Public(_)) => {
                (s.ident.to_string(), s.ident.span())
            }
            syn::Item::Enum(e) if matches!(e.vis, syn::Visibility::Public(_)) => {
                (e.ident.to_string(), e.ident.span())
            }
            syn::Item::Type(t) if matches!(t.vis, syn::Visibility::Public(_)) => {
                (t.ident.to_string(), t.ident.span())
            }
            syn::Item::Trait(t) if matches!(t.vis, syn::Visibility::Public(_)) => {
                (t.ident.to_string(), t.ident.span())
            }
            _ => continue,
        };
        if !Identifier(&name).is_stuttered_in(stem) {
            continue;
        }
        if !reexported.contains(&(stem.to_string(), name.clone())) {
            missing.push(rules::root_reexport::MissingReexport {
                line: span.start().line,
                ty: name,
            });
        }
    }
    missing
}

// no test_usage necessary
